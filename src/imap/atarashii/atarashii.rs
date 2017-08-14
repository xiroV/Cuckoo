extern crate atarashii_imap;
extern crate openssl;

use self::atarashii_imap::{Connection, Response};
use imap::{IMAPClient, IMAP};

impl IMAPClient for IMAP {
    fn connect() {
        println!("Connecting! (Test)");
    }
}

