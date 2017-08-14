/* The IMAP Interface
 *
 * New IMAP clients should implement the IMAPClient trait
 */

// Currently used IMAP client
pub mod atarashii;

use config::{Account};

pub trait IMAPClient {
    fn connect();
}

pub struct IMAP {}
