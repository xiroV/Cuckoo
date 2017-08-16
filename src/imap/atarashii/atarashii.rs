extern crate atarashii_imap;
extern crate openssl;

use self::atarashii_imap::{Connection, Response};
use imap::{IMAPClient, IMAP};
use self::openssl::ssl::{SslContext, SslStream, SslMethod, SslConnectorBuilder};
use std::io;

impl IMAPClient for IMAP {
    fn connect(server:&String, username:&String, password:&String) -> io::Result<()> {
        println!("Connecting..");
        match Connection::open_secure(server, username, password) {
            Ok(mut conn) => {
                // Try to select inbox
                match conn.select("inbox") {
                    Ok(sel_res) => {
                        let imap = IMAP {
                            connection: conn
                        };

                        Ok(())
                    },
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Connection Error"))
                }
            },
            _ => Err(io::Error::new(io::ErrorKind::Other, "Connection Error"))
        }
    }
}

