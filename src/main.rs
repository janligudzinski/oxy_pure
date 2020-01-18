#[macro_use]
extern crate log;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::core::Purifier;
use pretty_env_logger::init;
use std::time::Duration;

mod core;
use imap::error::Error;
use std::panic;
use std::borrow::{BorrowMut, Borrow};
use std::ops::Deref;

type ImapError = imap::error::Error;

lazy_static! {
    static ref PURIFIER: Mutex<Purifier> = {
        let pur = Purifier::new();
        Mutex::new(pur)
    };
}

fn main() {
    init();

    thread::spawn(move || {
        loop {
            {
                let mut purifier = PURIFIER.lock().unwrap();
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
