#[macro_use]
extern crate log;

use pretty_env_logger::init;
use std::fmt::Formatter;

mod core;
use crate::core::Purifier;

type ImapError = imap::error::Error;

#[derive(Debug)]
struct MyError {}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::core::fmt::Error> {
        write!(f, "placeholder type to satisfy Iron")?;
        Ok(())
    }
}
impl std::error::Error for MyError {}

fn main() {
    init();
    let mut purifier = Purifier::new();
    match purifier.run() {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    }
}
