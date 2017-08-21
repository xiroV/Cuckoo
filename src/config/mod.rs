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
    fn get_account(self, search_id: &String) -> Option<Account>;
}
