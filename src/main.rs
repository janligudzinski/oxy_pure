#[macro_use]
extern crate log;

use crate::core::Purifier;
use pretty_env_logger::init;
mod core;

fn main() {
    init();
    let purifier = Purifier::new();
}
