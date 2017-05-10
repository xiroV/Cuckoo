use std::env;
use std::path;
extern crate config as conf;

pub struct Account {
    pub id: String,
    pub name: String,
    pub mail: String,
    pub imap_server: String,
    pub imap_user: String,
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
        let mut conf_file = path::PathBuf::new();
        let mut conf = conf::Config::new();                   // config-rs

        // Find home directory
        match env::home_dir() { 
            Some(path) => { conf_file = path; },
            None => println!("Unable to find home directory. Config file not found")
        }

        // Construct path to config file
        conf_file.push(".config");
        conf_file.push("cuckoo");
        conf_file.push("config.toml");

        println!("Reading config file: {:?}", conf_file);
        
        // Get values from config file
        match conf.merge(conf::File::new(conf_file.to_str().unwrap(), conf::FileFormat::Toml)) {
            Ok(_) => { },
            Err(_) => { return Some("Config file not found"); },
        }

        //conf.merge(conf::File::new(conf_file.to_str().unwrap(), conf::FileFormat::Toml)).unwrap();

        // Loop through accounts
        for (account_id, values) in conf.get("accounts").unwrap().into_table().unwrap() { 
            let mut account_info = values.into_table().unwrap();

            // Generate the account structure 
            let account = Account {
                id: account_id,
                name: account_info.remove("name").unwrap().into_str().unwrap(),
                mail: account_info.remove("mail").unwrap().into_str().unwrap(),
                imap_server: account_info.remove("imap_server").unwrap().into_str().unwrap(),
                imap_user: account_info.remove("imap_user").unwrap().into_str().unwrap(),
            };

            // Push to the config structure
            self.accounts.push(account);
        }
        return None; 
    }
}
