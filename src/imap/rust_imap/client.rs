extern crate imap;
extern crate openssl;

use imap::{IMAP, IMAPClient, IMAPError};
use imap::rust_imap::Connection;
use self::openssl::ssl::{SslMethod, SslConnectorBuilder, SslStream};
use std::io;
use std::net::TcpStream;
use self::imap::client::Client;

impl IMAPClient for IMAP<Connection> {
    fn connect(server:&String, username:&String, password:&String) -> Result<Self, IMAPError> {
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
            Ok(_) => {
                let imap = Self {
                    connection: imap_socket,
                };
                return Ok(imap)
            },
            Err(_) => return Err(IMAPError::Login)
        };
    }
}

