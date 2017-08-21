/* The IMAP Interface
 *
 * New IMAP clients should implement the IMAPClient trait
 */

//extern crate imap;
//extern crate openssl;

// Currently used IMAP client
pub mod rust_imap;
pub use imap::rust_imap::Connection;

/*
use self::openssl::ssl::{SslConnectorBuilder, SslMethod, SslStream};
use self::imap::client::Client;
use std::net::TcpStream;
*/

use config::{Account};

pub trait IMAPClient {
    fn connect(&String, &String, &String) -> Result<IMAP<Connection>, IMAPError>;
    fn capability(&mut self); // -> Result<Vec<String>>;
    // fn fetch(&mut self, mailbox: &String) -> Result<Vec<&String>, String>;
    fn disconnect(&mut self);
}

pub struct IMAP<T> {
    connection: T
}

pub enum IMAPError {
    Connection,
    Login,
}
