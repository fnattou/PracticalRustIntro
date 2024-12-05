use actix_web::{get, App, HttpResponse, HttpServer, ResponseError, web, web::Data};
use thiserror::Error;
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

struct TodoEntry{
    id: u32,
    text: String,
}
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Error, Debug)]
enum MyError{
    #[error("Failed to render HTML")]
    AskamaErorr(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}
impl  ResponseError for MyError {}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;

    // SQL文をPrepared Statementに変換
    let mut statement = conn.prepare("SELECT id , text FROM todo")?;

    // Prepared StatementになっているSQL文を実行し、結果をTodoEntryに変換する
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry {id, text})
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }
    let html = IndexTemplate {entries};
    let response_body = html.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error>{
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to inititalize the connection pool.");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![])
        .expect("Failed to create a table 'todo'.");
    HttpServer::new(move || App::new().service(index).app_data(Data::new(pool.clone())))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
