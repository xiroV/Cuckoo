/* The Config Interface
 *
 * To implement a new config parser the parser should implement the
 * ConfigReader for the Config structure, and to use the new interface
 * the `mod ...` line in this file should be changed to the name of
 * the new config parser module.
 */

mod configrs;

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
    fn init() -> Self;
    fn read(&mut self) -> Result<&Self, ConfigError>;
    fn get_account(self, search_id: &String) -> Option<Account>;
}

pub enum ConfigErrorType {
    MissingField,
    FileNotFound,
}

pub struct ConfigError {
    pub err_type: ConfigErrorType,
    pub field: Option<String>,
}

pub trait ConfigErrorHandler {
    fn missing_field(field: &str) -> Self;
    fn file_not_found() -> Self;
}
