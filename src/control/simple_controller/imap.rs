use imap::{IMAP, IMAPClient, IMAPErrorType, IMAPErrorHandler};
use control::{Control, Controller, IMAPController, ControlMessage, MessageHandler, MessageType};
use config::{Config, ConfigReader, ConfigError, ConfigErrorType};

impl IMAPController for Control {
    fn imap_connect(&mut self, account: &String, password: &String) -> Result<&Self, ControlMessage> {
        let mut conf = Config::init();

        match conf.read() {
            Ok(_) => { },
            Err(e) => {
                match e.err_type {
                    ConfigErrorType::MissingField => {
                        return Err(
                            ControlMessage::new(
                                MessageType::error,
                                &format!("Missing account field '{:?}' in config file", e.field)
                            )
                        )
                    },
                    ConfigErrorType::FileNotFound => {
                        return Err(
                            ControlMessage::new(
                                MessageType::error,
                                "Config file not found"
                            )
                        )
                    }
                };
            }
        }

        // FIXME This is a mess
        match conf.get_account(account) {
            Some(acc) => {
                self.push_message(ControlMessage::new(MessageType::message, "Connecting.."));

                match IMAP::connect(&acc.imap_server, &acc.imap_user, &password) {
                    Ok(conn) => {
                        self.imap_connection = Some(conn);
                        return Ok(self);
                    },
                    Err(e) => match e.err_type {
                        IMAPErrorType::Connection => Err(
                            ControlMessage::new(
                                MessageType::error,
                                "Connection Failed"
                            )
                        ),
                        IMAPErrorType::Login => Err(
                            ControlMessage::new(
                                MessageType::error,
                                "Login failed"
                            )
                        ),
                    },
                }
            },
            None => {
                return Err(
                    ControlMessage::new(
                        MessageType::error,
                        "account not found in config"
                    )
                );
            }
        }
    }
}
