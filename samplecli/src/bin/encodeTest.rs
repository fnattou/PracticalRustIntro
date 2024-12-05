
fn main () {
    #[cfg(windows)]
    {
        use std::ffi::OsString;
        use std::os::windows::prelude::*;

        let source = [0x0061, 0x0062, 0xD800];
        let os_string = OsString::from_wide(&source[..]);
        let os_str = os_string.as_os_str();

        println!("{}", os_str.to_string_lossy());
        println!("{}", os_str.to_str().unwrap());
    }

}