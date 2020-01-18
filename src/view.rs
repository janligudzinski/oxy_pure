use ramhorns::{Template, Content};
use crate::core::Purifier;

#[derive(Content)]
pub struct Info {
    since: String,
    counter: usize
}

impl Purifier {
    pub fn info(&self) -> Info {
        Info {
            since: self.since().to_rfc3339(),
            counter: self.counter()
        }
    }
}

impl Info {
    pub fn json(&self) -> String {
        format!("{{\"since\": \"{}\", \"counter\": {}}}", &self.since, self.counter)
    }
}

pub fn view(info: &Info) -> String {
    Template::new(include_str!("index.html")).unwrap().render(info)
}