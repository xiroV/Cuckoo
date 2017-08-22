use imap::{IMAP, Connection, IMAPError}

mod simple_controller;

struct Control {
    imap_connection: Some(IMAP<Connection>),
}

trait Controller {
    fn new() -> Self;
    fn imap_connect() -> Result<Self, ()>; // FIXME Should not return () on error
}
