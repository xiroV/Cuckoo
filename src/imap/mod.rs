/* The IMAP Interface
 *
 * New IMAP clients should implement the IMAPClient trait
 */

// Currently used IMAP client
pub mod rust_imap;

use config::{Account};
use std::io;

pub trait IMAPClient {
    fn connect(&String, &String, &String) -> Result<(), IMAPError>;
}

pub struct IMAP {}

pub enum IMAPError {
    Connection,
    Login
}
