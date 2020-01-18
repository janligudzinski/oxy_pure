#[macro_use]
extern crate log;

use std::sync::Mutex;
use std::thread;
use crate::core::Purifier;
use pretty_env_logger::init;
use std::time::Duration;

mod core;
use imap::error::Error;
use std::panic;

type ImapError = imap::error::Error;

fn main() {
    init();
    let mut purifier = Mutex::new(Purifier::new());
    thread::spawn(move || {
        loop {
            {
                let purifier = purifier.get_mut().unwrap();
                purifier.run().err().and_then(|im| {
                    match im {
                        Error::No(msg) => {
                            error!("The server returned a NO response.");
                            error!("This is probably an internal error, so the program will wait and try later.");
                            error!("{}", msg);
                        },
                        _ => {
                            error!("{}", im);
                            panic!("An error occurred. The oxy_pure worker thread will now stop.")
                        }
                    };
                    Some(())
                });
            }
            thread::sleep(Duration::from_millis(2000));
        }
    }).join().unwrap();
}
