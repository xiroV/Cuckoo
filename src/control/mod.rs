use imap::{IMAP, Connection, IMAPError}

mod simple_controller;

enum MessageType {
    message,
    error, 
}

struct Control {
    imap_connection: Some(IMAP<Connection>),
}

struct ControlMessage {
    msg_type: 
    description: String;
}

trait Controller {
    fn new() -> Self;
    fn imap_connect(&self, &String) -> Result<Self, ControlMessage>;
}

trait MessageHandler {
    fn new(&str) -> Self;
}
