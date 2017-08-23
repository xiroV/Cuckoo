mod simple_controller;

use imap::{IMAP, Connection};

enum MessageType {
    Message,
    Error, 
}

struct ControlMessage {
    message_type: MessageType,
    description: String,
}

pub struct Control {
    imap_connection: Option<IMAP<Connection>>,
    message: Option<ControlMessage>,
}

pub trait Controller {
    fn new() -> Self;
    fn push_message(&mut self, ControlMessage) -> &Self;
}

pub trait IMAPController {
    fn imap_connect(&mut self, account: &String, password: &String) -> Result<&Self, ControlMessage>;
}

trait MessageHandler {
    fn new(MessageType, &str) -> Self;
}
