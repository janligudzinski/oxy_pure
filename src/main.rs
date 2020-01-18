#[macro_use]
extern crate log;

use std::sync::Mutex;
use std::thread;
use crate::core::Purifier;
use pretty_env_logger::init;
use std::time::Duration;

mod core;
type ImapError = imap::error::Error;

fn main() {
    init();
    let mut purifier = Mutex::new(Purifier::new());
    thread::spawn(move || {
        loop {
            {
                let purifier = purifier.get_mut().unwrap();
                if let Err(e) = purifier.run() {
                    println!("{}", e);
                }
            }
            thread::sleep(Duration::from_millis(2000));
        }
    }).join().unwrap();
}
