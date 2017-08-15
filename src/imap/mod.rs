/* The IMAP Interface
 *
 * New IMAP clients should implement the IMAPClient trait
 */

extern crate atarashii_imap;

// Currently used IMAP client
pub mod atarashii;

use config::{Account};
use self::atarashii_imap::{Connection};
use std::io;

pub trait IMAPClient {
    fn connect(&String, &String, &String) -> io::Result<()>;
}

pub struct IMAP {
    connection: Connection,
}
