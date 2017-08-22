use imap::{IMAP, IMAPClient, IMAPErrorType, IMAPErrorHandler};
use control::{Control, IMAPController, ControlMessage, MessageHandler, MessageType};
use config::{Config, ConfigReader};

impl IMAPController for Control {
    fn imap_connect(&mut self, account: &String, password: &String) -> Result<&Self, ControlMessage> {
        let mut conf = Config::new();

        match conf.read() {
            None => { },
            Some(err) => println!("{}", err)
        }

        // FIXME This is a mess
        match conf.get_account(account) {
            Some(acc) => {
                match acc.imap_server {
                    Some(imap_server) => {
                        match acc.imap_user {
                            Some(imap_user) => {
                                println!("Connecting..");

                                match IMAP::connect(&imap_server, &imap_user, &password) {
                                    Ok(conn) => {
                                        self.imap_connection = Some(conn);
                                        return Ok(self);
                                    },
                                    Err(e) => match e.err_type {
                                        IMAPErrorType::Connection => Err(
                                            ControlMessage::new(
                                                MessageType::error,
                                                "Connection Failed")
                                            ),
                                        IMAPErrorType::Login => Err(
                                            ControlMessage::new(
                                                MessageType::error,
                                                "Login failed"
                                            )
                                        ),
                                    }
                                }
                            },
                            None => {
                                return Err(ControlMessage::new(MessageType::error, "IMAP user not found in config"));
                            }
                        }
                    },
                    None => {
                        return Err(ControlMessage::new(MessageType::error, "imap server not found in config"));
                    }
                }
            },
            None => {
                return Err(ControlMessage::new(MessageType::error, "account not found in config"));
            }
        }
    }
}
