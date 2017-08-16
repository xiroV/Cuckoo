extern crate imap;
extern crate openssl;

use self::imap::client::Client;
use imap::{IMAP, IMAPClient, IMAPError};
use self::openssl::ssl::{SslMethod, SslConnectorBuilder};
use std::io;

impl IMAPClient for IMAP {
    fn connect(server:&String, username:&String, password:&String) -> Result<(), IMAPError> {
        println!("Connecting..");

        let port = 993;
        let socket_addr = (server.as_str(), port);
        let ssl_connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap().build();
        
        // Connect to mail server
        let mut imap_socket = match Client::secure_connect(socket_addr, server, ssl_connector) {
            Ok(s) => s,
            Err(_) => return Err(IMAPError::Connection)   
        };

        // Login to mail server
        match imap_socket.login(username, password) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(IMAPError::Login)
        };
    }
}

