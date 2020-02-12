use std::net::TcpStream;

use chrono::{DateTime, Utc};
use imap::{connect, Session};
use native_tls::{TlsConnector, TlsStream};
use rustyknife::rfc2047::encoded_word;

use super::ImapError;

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
            username: var("O2_USERNAME")
                .expect("The O2_USERNAME environment variable must be set."),
            password: var("O2_PASSWORD")
                .expect("The O2_PASSWORD environment variable must be set."),
        }
    }
    fn session(&self) -> Result<ImapSession, ImapError> {
        let tls = TlsConnector::builder().build().unwrap();
        let client = connect((SERVER, PORT), SERVER, &tls)?;
        info!("Successfully connected to poczta.o2.pl");
        match client.login(&self.username, &self.password) {
            Ok(ses) => Ok(ses),
            Err(e) => Err(e.0)
        }
    }
    fn get_spam_uids(&self, session: &mut ImapSession) -> Result<String, ImapError> {
        session.select("INBOX")?;
        let mut sequence = String::new();
        info!("Fetching messages...");
        for msg in session.fetch("1:*", "UID ENVELOPE")?.into_iter() {
            let envelope = match msg.envelope() {
                Some(env) => env,
                None => continue,
            };
            let from = match &envelope.from {
                Some(from) => from,
                None => continue,
            };
            let uid = match msg.uid {
                Some(u) => u,
                None => continue,
            };
            for address in from {
                if let Some(name) = address.name {
                    let (_, name) = encoded_word(name.as_bytes()).unwrap();
                    if name.contains("/o2") || name.contains("/ o2") {
                        info!(
                            "Found a spam message with the sender name {:?} and UID {}",
                            name, uid
                        );
                        sequence.push_str(&format!("{},", uid));
                    }
                }
            }
        }
        sequence.pop(); // remove the last trailing comma
        Ok(sequence)
    }
    fn delete_messages(
        &mut self,
        sequence: &str,
        session: &mut ImapSession,
    ) -> Result<usize, ImapError> {
        if sequence.is_empty() {
            return Ok(0);
        }
        let mut count = 0usize;
        count += sequence.split(",").count();
        session.uid_store(sequence, "+FLAGS (\\Deleted)")?;
        info!("Set the Deleted flag on {} messages.", count);
        session.expunge()?;
        info!("Expunged the mailbox.");
        info!("Success, {} messages permanently deleted.", count);
        //TODO add optional DB persistence for the counter
        Ok(count)
    }
    pub fn run(&mut self) -> Result<usize, ImapError> {
        let now = Utc::now().to_rfc3339();
        info!("oxy_pure run at {}", now);
        info!("Acquiring session...");
        let session = &mut self.session()?;
        let spam = self.get_spam_uids(session)?;
        let count = self.delete_messages(&spam, session)?;
        info!("Logging out...");
        session.logout().ok(); // the imap crate can't handle an "a4 BYE IMAP4rev1 Server logging out" response
        info!("Logged out.");
        Ok(count)
    }
}
