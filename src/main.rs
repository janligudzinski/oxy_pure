#[macro_use]
extern crate log;
use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use crate::core::Purifier;
use pretty_env_logger::init;
use std::time::Duration;

mod core;
mod view;
use std::panic;

type ImapError = imap::error::Error;

lazy_static! {
    static ref PURIFIER: Mutex<Purifier> = {
        let pur = Purifier::new();
        Mutex::new(pur)
    };
}


use iron::prelude::*;
use iron::{status, IronError};
use iron::{mime, mime::{Mime, SubLevel, TopLevel}};
use crate::view::Info;
use std::fmt::Formatter;

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
    dbg!(&path);
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
            {
                let mut purifier = PURIFIER.lock().unwrap();
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
            thread::sleep(Duration::from_millis(2000));
        }
    }).join().unwrap();
}
