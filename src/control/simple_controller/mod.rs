mod imap;
mod message;
use control::{Control, Controller, ControlMessage};

impl Controller for Control {
    fn new() -> Self {
        Control {
            imap_connection: None,
            message: None,
        }
    }

    fn push_message(&mut self, message: ControlMessage) -> &Self {
        self.message = Some(message);
        return self;
    }
}
