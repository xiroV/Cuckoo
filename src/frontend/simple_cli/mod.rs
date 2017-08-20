mod simple_cli;
mod control;
mod messages;
mod model;

use imap::{IMAP, Connection};

pub struct SimpleCli {
    imap_connection: Option<IMAP<Connection>>,
}
