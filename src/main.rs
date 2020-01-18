#[macro_use]
extern crate log;

use std::fmt::Formatter;
use std::panic;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use iron::{IronError, status};
use iron::mime::Mime;
use iron::prelude::*;
use pretty_env_logger::init;

use lazy_static::lazy_static;

use crate::core::Purifier;
mod core;
mod view;

type ImapError = imap::error::Error;

lazy_static! {
    static ref PURIFIER: Mutex<Purifier> = {
        let pur = Purifier::new();
        Mutex::new(pur)
    };
}


#[derive(Debug)]
struct MyError {}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::core::fmt::Error> {
        write!(f, "placeholder type to satisfy Iron")?;
        Ok(())
    }
}
impl std::error::Error for MyError {

}

fn handler(req: &mut Request) -> IronResult<Response> {
    let purifier = PURIFIER.lock().unwrap();
    let path = req.url.path();
    if path.len() > 1 {
        Err(IronError::new(MyError {}, (status::NotFound, "NOT FOUND")))
    } else if path[0] == "info" {
        let json: Mime = "application/json".parse().unwrap();
        Ok(Response::with((status::Ok, json, format!("{}", purifier.info().json()))))
    } else if path[0].is_empty() {
        let html: Mime = "text/html".parse().unwrap();
        Ok(Response::with((status::Ok, html, view::view(&purifier.info()))))
    } else {
        Err(IronError::new(MyError {}, (status::NotFound, "NOT FOUND")))
    }
}

fn main() {
    init();
    let _server = Iron::new(handler).http("localhost:3000").unwrap();
    thread::spawn(move || {
        loop {
            let period: u64;
            {
                let mut purifier = PURIFIER.lock().unwrap();
                period = purifier.wait_period();
                purifier.run().err().and_then(|im| {
                    match im {
                        ImapError::No(msg) => {
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
            thread::sleep(Duration::from_millis(period));
        }
    }).join().unwrap();
}
