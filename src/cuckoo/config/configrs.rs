/*
 * Implementation of ConfigReader using the config-rs crate
 */

use cuckoo::config::{Account, Config, ConfigReader, ConfigError};
use std::env;
use std::collections::HashMap;
extern crate config as conf;

fn cfg_as_str(table: &mut HashMap<String, conf::Value>, key: &str,
                             readable_key_prefix: &str) -> Result<String, ConfigError> {
    let error = || ConfigError::field_error(&format!("{}.name", readable_key_prefix), "string");
    let value = table.remove(key).ok_or_else(&error)?;
    value.into_str().map_err(|_| error())
}

impl ConfigReader for Config {
    fn new() -> Self {
        let conf = Config { accounts: Vec::new() };
        return conf;
    }

    fn read(&mut self) -> Result<(), ConfigError> {
        let mut conf = conf::Config::new();

        let mut conf_path = env::home_dir()
            .ok_or_else(|| ConfigError::file_not_found(
                    "$HOME",
                    "Could not find home directory, needed for locating configuration file."
            ))?;

        // Construct path to config file
        conf_path.push(".config");
        conf_path.push("cuckoo");
        conf_path.push("config.toml");

        //println!("Reading config file: {:?}", conf_path);
        let conf_file = conf::File::new(conf_path.to_str().unwrap(), conf::FileFormat::Toml);

        // Get values from config file
        conf.merge(conf_file).map_err(|_| ConfigError::file_not_found(
                conf_path.as_path().to_str().unwrap_or_default(),
                "Could not find configuration file"
        ))?;

        let accounts = conf.get_table("accounts")
            .map_err(|e| ConfigError::field_error_with_cause(
                    "accounts", "map", &format!("{}", e)))?;

        for (account_id, values) in accounts {
            let readable_account_key = format!("accounts['{}']", account_id);
            let mut acc_table = values.into_table().map_err(|_| ConfigError::field_error(
                    &readable_account_key, "account"
            ))?;

            {
                // Parse an account from 'acc_table
                let name = cfg_as_str(&mut acc_table, "name", &readable_account_key)?;
                let mail = cfg_as_str(&mut acc_table, "mail", &readable_account_key)?;
                let imap_server = cfg_as_str(&mut acc_table, "imap_server", &readable_account_key)?;
                let imap_user = cfg_as_str(&mut acc_table, "imap_user", &readable_account_key)?;

                // TODO if account_info.size != 0 emit warning.

                let account = Account {
                    id: account_id,
                    name: name,
                    mail: mail,
                    imap_server: imap_server,
                    imap_user: imap_user
                };

                self.accounts.push(account);
            }
        }
        return Ok(());
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
