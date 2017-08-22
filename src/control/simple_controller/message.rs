use control::{MessageHandler, ControlMessage, MessageType};

impl MessageHandler for ControlMessage {
    fn new(msg_type: MessageType, desc: &str) -> Self {
        ControlMessage {
            message_type: msg_type,
            description: String::from(desc),
        }
    }
}
