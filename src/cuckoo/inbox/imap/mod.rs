extern crate imap;
extern crate openssl;

use self::imap::client::Client;
use self::openssl::ssl::SslStream;
use std::net::TcpStream;

mod client;
mod error;

// Type of connection used by the IMAP client
pub type Connection = Client<SslStream<TcpStream>>;

/*
 * New IMAP clients should implement the IMAPClient trait
 */
pub trait IMAPClient {
    fn connect(hostname: &str, username: &str,
               password: &str) -> Result<IMAP<Connection>, IMAPError>;

    fn capability(&mut self); // -> Result<Vec<String>>;

    // fn fetch(&mut self, mailbox: &String) -> Result<Vec<&String>, String>;

    fn disconnect(&mut self);
}

pub struct IMAP<T> {
    connection: T
}

pub enum IMAPErrorType {
    Connection,
    Login,
}

pub struct IMAPError {
    pub err_type: IMAPErrorType,
}

pub trait IMAPErrorHandler {
    fn new(error_type: IMAPErrorType) -> Self;
}

