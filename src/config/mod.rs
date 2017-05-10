mod configrs;

#[derive(Default)]
pub struct Account {
    pub id: String,
    pub name: Option<String>,
    pub mail: Option<String>,
    pub imap_server: Option<String>,
    pub imap_user: Option<String>,
}

pub struct Config {
    pub accounts: Vec<Account>,    
}

pub trait ConfigReader {
    fn new() -> Self;
    fn read(&mut self) -> Option<&str>;
}
