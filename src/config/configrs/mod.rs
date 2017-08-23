/* Implementation of ConfigReader using the config-rs crate
 */

mod error;
use config::{Account, Config, ConfigReader, ConfigError, ConfigErrorType, ConfigErrorHandler};
use std::env;
//use std::path;
extern crate config as conf;

impl ConfigReader for Config {
    fn init() -> Self {
        let conf = Config { accounts: Vec::new() };
        return conf;
    }

    fn read(&mut self) -> Result<&Self, ConfigError> {
        let mut conf_path;
        let mut conf = conf::Config::new();                   // config-rs

        match env::home_dir() {
            Some(path) => { conf_path = path; },
            None => { 
                return Err(ConfigError::file_not_found());
            }
        }

        // Construct path to config file
        conf_path.push(".config");
        conf_path.push("cuckoo");
        conf_path.push("config.toml");

        //println!("Reading config file: {:?}", conf_path);

        let conf_file = conf::File::new(conf_path.to_str().unwrap(), conf::FileFormat::Toml); 

        // Get values from config file
        match conf.merge(conf_file) {
            Ok(_) => { },
            Err(_) => {
                return Err(ConfigError::file_not_found())
            },
        }

        // Loop through accounts
        for (account_id, values) in conf.get("accounts").unwrap().into_table().unwrap() { 
            let mut account_info = values.into_table().unwrap();

            // Generate the account structure 
            let mut account = Account {
                id: account_id,
                name: match account_info.remove("name") {
                    Some(name) => name.into_str().unwrap(),
                    None => {
                        return Err(ConfigError::missing_field("name"))
                    },
                },
                mail: match account_info.remove("mail") {
                    Some(mail) => mail.into_str().unwrap(),
                    None => {
                        return Err(ConfigError::missing_field("mail"))
                    },
                },
                imap_server: match account_info.remove("imap_server") {
                    Some(imap_server) => imap_server.into_str().unwrap(),
                    None => {
                        return Err(ConfigError::missing_field("imap_server"))
                    }
                },
                imap_user: match account_info.remove("imap_user") {
                    Some(imap_user) => imap_user.into_str().unwrap(),
                    None => {
                        return Err(ConfigError::missing_field("imap_user"))
                    }
                },
            };

            // Push to the config structure
            self.accounts.push(account);
        }

        return Ok(self); 
    }

    fn get_account(self, search_id: &String) -> Option<Account> {
        for acc in self.accounts {
            if &acc.id == search_id {
                return Some(acc);
            }
        }
        return None;
    }
}
