use std::env;
use std::path;
extern crate config as conf;

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


impl ConfigReader for Config {
    fn new() -> Self {
        let conf = Config { accounts: Vec::new() };
        return conf;
    }

    fn read(&mut self) -> Option<&str> {
        let mut conf_path = path::PathBuf::new();
        let mut conf = conf::Config::new();                   // config-rs

        // Find home directory
        match env::home_dir() { 
            Some(path) => { conf_path = path; },
            None => println!("Unable to find home directory. Config file not found")
        }

        // Construct path to config file
        conf_path.push(".config");
        conf_path.push("cuckoo");
        conf_path.push("config.toml");

        println!("Reading config file: {:?}", conf_path);

        let conf_file = conf::File::new(conf_path.to_str().unwrap(), conf::FileFormat::Toml); 

        // Get values from config file
        match conf.merge(conf_file) {
            Ok(_) => { },
            Err(_) => { return Some("Config file not found"); },
        }

        // Loop through accounts
        for (account_id, values) in conf.get("accounts").unwrap().into_table().unwrap() { 
            let mut account_info = values.into_table().unwrap();

            // Generate the account structure 
            let account = Account {
                id: account_id,
                name: match account_info.remove("name") {
                    Some(name) => Some(name.into_str().unwrap()),
                    None => None,
                },
                mail: match account_info.remove("mail") {
                    Some(mail) => Some(mail.into_str().unwrap()),
                    None => None,
                },
                imap_server: match account_info.remove("imap_server") {
                    Some(imap_server) => Some(imap_server.into_str().unwrap()),
                    None => None,
                },
                imap_user: match account_info.remove("imap_user") {
                    Some(imap_user) => Some(imap_user.into_str().unwrap()),
                    None => None,
                },
            };

            // Push to the config structure
            self.accounts.push(account);
        }
        return None; 
    }
}
