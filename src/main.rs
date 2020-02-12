#[macro_use]
extern crate log;

use pretty_env_logger::init;
use std::fmt::Formatter;

mod core;
use crate::core::Purifier;

type ImapError = imap::error::Error;

fn main() {
    init();
    let mut purifier = Purifier::new();
    match purifier.run() {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    }
}
