use imap::{connect, Client, Session};
use native_tls::{TlsConnector, TlsStream};
use std::error::Error;
use std::net::TcpStream;
use std::fmt::Display;
use rustyknife::rfc2047::encoded_word;
use imap::types::{Mailbox, Fetch};

type ImapError = imap::error::Error;
type ImapSession = Session<TlsStream<TcpStream>>;

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
    fn session(&self) -> ImapSession {
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
    pub fn get_spam(&self, session: &mut ImapSession) -> Result<Vec<&Fetch>, ImapError> {
        get_inbox(session)?;
        info!("Fetching messages...");
        let fetched = session.fetch("1:*", "UID ENVELOPE")?;
        let count = fetched.iter().count();
        info!("Found {} messages in the inbox.", count);
        let messages = fetched.iter().filter(|msg| {
            let envelope = match msg.envelope() {
                Some(env) => env,
                None => return false
            };
            let from = match &envelope.from {
                Some(from) => from,
                None => return false
            };
            for address in from {
                if let Some(name) = address.name {
                    let (_, name) = encoded_word(name.as_bytes()).unwrap();
                    if name.contains("/o2") || name.contains("/ o2") {
                        return true
                    }
                }
            }
            false
        });
        let messages = messages.collect::<Vec<_>>();
        for message in messages.iter() {
            info!("Found spam e-mail with UID {:#?}", message.uid);
            if let Some(env) = message.envelope() {
                if let Some(from) = &env.from {
                    info!("addresses: {:#?}", from);
                };
                if let Some(sub) = &env.subject {
                    info!("subject: {:#?}", sub);
                }
            }
        }
        Ok(messages)
    }
}

fn get_inbox(session: &mut ImapSession) -> Result<Mailbox, ImapError> {
    match session.select("INBOX") {
        Ok(mailbox) => {
            info!("Successfully got the inbox.");
            Ok(mailbox)
        },
        Err(e) => {
            match e {
                ImapError::No(msg) => {
                    warn!("The IMAP server responded with a NO. You should probably try again later");
                    warn!("Error message: {}", &msg);
                    return Err(ImapError::No(msg))
                },
                _ => {
                    error!("The INBOX mailbox could not be gotten.");
                    error!("Error message: {:#?}", e);
                    std::process::exit(4);
                }
            }
        }
    }
}