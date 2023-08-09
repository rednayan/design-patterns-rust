// https://web.archive.org/web/20201112023149/https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
// building interfaces which accepts string
// solving the problem of always using "to_string()"  which is analagous of using ".clone()" to get around with the borrow checker
// DEREF COERCION

use core::fmt;
use std::error::Error;

#[derive(Debug)]
struct FunError;

impl Error for FunError {}

impl fmt::Display for FunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

fn print_me(msg: &str) -> Result<(), FunError> {
    println!("{msg}");
    Ok(())
}

fn main() -> Result<(), FunError> {
    let owned_string = "hello world".to_string();
    print_me(&owned_string)?;

    let counted_string = std::rc::Rc::new("hello world".to_string());
    print_me(&counted_string)?;

    let atomically_counted_string = std::sync::Arc::new("hello world".to_string());
    print_me(&atomically_counted_string)?;

    Ok(())
}
