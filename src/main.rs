#[macro_use]
extern crate log;

use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use lambda_runtime::{error::HandlerError, lambda};
use pretty_env_logger::init;
use std::error::Error;
use std::fmt::Formatter;
mod core;
use crate::core::Purifier;
use serde::Serialize;
type ImapError = imap::error::Error;

// The program will be triggered by AWS CloudWatch timer events on a schedule.
fn handler(_ev: CloudWatchEvent, _c: lambda_runtime::Context) -> Result<usize, HandlerError> {
    let mut purifier = Purifier::new();
    match purifier.run() {
        Ok(a) => Ok(a),
        Err(e) => Err(HandlerError::from(format!("{}", e).as_str()))
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    init();
    lambda!(handler);
    Ok(())
}
