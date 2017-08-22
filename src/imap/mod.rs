/* The IMAP Interface
 *
 * New IMAP clients should implement the IMAPClient trait
 */

// Currently used IMAP client and connection
pub mod rust_imap;
pub use imap::rust_imap::Connection;

pub trait IMAPClient {
    fn connect(&String, &String, &String) -> Result<IMAP<Connection>, IMAPError>;
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
    fn new(IMAPErrorType) -> Self;
}
