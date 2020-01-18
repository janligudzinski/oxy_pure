use imap::{connect, Client, Session};
use native_tls::{TlsConnector, TlsStream};
use std::error::Error;
use std::net::TcpStream;
use std::fmt::Display;

const SERVER: &str = "poczta.o2.pl";
const PORT: u16 = 993;

pub struct Purifier {
    /// The username used to connect to the server.
    username: String,
    /// The password used to connect to the server.
    password: String,
}

impl Purifier {
    pub fn new() -> Self {
        use std::env::var;
        Self {
            username: var("O2_USERNAME").expect("The O2_USERNAME environment variable must be set."),
            password: var("O2_PASSWORD").expect("The O2_PASSWORD environment variable must be set.")
        }
    }
    pub fn session(&self) -> Session<TlsStream<TcpStream>> {
        let tls = TlsConnector::builder().build().unwrap();
        let client = match connect((SERVER, PORT), SERVER, &tls) {
            Ok(client) => {
                info!("Successfully connected to poczta.o2.pl");
                client
            },
            Err(e) => {
                error!("Failed to connect to poczta.o2.pl, exiting!");
                error!("Error message: {:#?}", e);
                std::process::exit(1)
            }
        };
        match client.login(&self.username, &self.password) {
            Ok(session) => {
                info!("Successfully authenticated.");
                session
            },
            Err(e) => {
                error!("Could not log in, exiting!");
                error!("Error message: {:#?}", e);
                std::process::exit(2)
            }
        }
    }
}