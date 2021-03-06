extern crate imap;
extern crate openssl;

use super::{IMAP, IMAPClient, IMAPError, IMAPErrorType, IMAPErrorHandler};
use super::Connection;
use self::openssl::ssl::{SslMethod, SslConnectorBuilder};
use self::imap::client::Client;

impl IMAPClient for IMAP<Connection> {
    fn connect(server: &str, username: &str, password: &str) -> Result<Self, IMAPError> {
        let port = 993;
        let socket_addr = (server, port);
        let ssl_connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap().build();

        // Connect to mail server
        let mut imap_socket = match Client::secure_connect(socket_addr, server, ssl_connector) {
            Ok(s) => s,
            Err(_) => return Err(IMAPError::new(IMAPErrorType::Connection))
        };

        // Login to mail server
        match imap_socket.login(username, password) {
            Ok(_) => {
                let imap = Self {
                    connection: imap_socket,
                };
                return Ok(imap);
            }
            Err(_) => return Err(IMAPError::new(IMAPErrorType::Login))
        }
    }

    // FIXME Should not print from here, but return list of capabilities
    fn capability(&mut self) {
        match self.connection.capability() {
            Ok(capabilities) => {
                for capability in capabilities.iter() {
                    println!("{}", capability);
                }
            }
            Err(e) => println!("Error parsing capability: {}", e),
        };
    }

    /*fn fetch(&mut self, mailbox:&String) -> Result<Vec<&String>, String> {

        let mut mail_list: Vec<&String> = Vec::new();

        // Select inbox
        match self.connection.select(mailbox) {
            Ok(r) => {
                println!("Result of Select: {}", r);

                // Fetch all mail
                match self.connection.fetch("*", "all") {
                    Ok(lines) => {
                        for line in lines.iter() {
                            mail_list.push(&line);
                        }

                        return Ok(mail_list);
                    },
                    Err(e) => return Err(String::from("Error Fetching emails")),
                };
            },
            Err(e) => return Err(String::from("Connection Error")),
        };

    }*/

    fn disconnect(&mut self) {
        self.connection.logout().unwrap();
    }
}

