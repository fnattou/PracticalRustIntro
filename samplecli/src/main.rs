use anyhow::{bail, ensure, Context, Result};

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::path::PathBuf;

struct RpnCalculator(bool);
impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }
    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("Invalid syntax");
                let x = stack.pop().expect("Invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("Invalid token")
                };
                stack.push(res);
            }
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");
        Ok(stack[0])
    }
}


#[derive(Parser, Debug)]
#[command(
    name = "My RPN program",
    version = "1.0.0",
    author = "Sota Funato",
    about = "Super awesome sample RPN calcurator")]
struct Cli {
    ///Sets the level of verbosity
    #[arg(short, long, default_value="false")]
    verbose: bool, 

    ///Formulas written in RPN
    #[arg(name = "FILE")]
    formula_file: Option<PathBuf>
}

fn main() {
    let args = Cli::parse();
    if let Some(path) = args.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, args.verbose);
    } else {
        println!("No file is specified");
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, args.verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()>{
    let calc = RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("50"), 50);
        assert_eq!(calc.eval("-50"), -50);

        assert_eq!(calc.eval("2 3 +"), 5);
        assert_eq!(calc.eval("2 3 *"), 6);
        assert_eq!(calc.eval("2 3 -"), -1);
        assert_eq!(calc.eval("2 3 /"), 0);
        assert_eq!(calc.eval("2 3 %"), 2);
    }

    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        calc.eval("1 1 ^");
    }
}