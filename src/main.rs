#[macro_use]
extern crate log;

use crate::core::Purifier;
use pretty_env_logger::init;
mod core;
type ImapError = imap::error::Error;

fn main() {
    init();
    let mut purifier = Purifier::new();
    if let Err(e) = purifier.run() {
           error!("{:?}", e);
    }
}
