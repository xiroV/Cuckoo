extern crate imap;
extern crate openssl;

use self::imap::client::Client;
use self::openssl::ssl::{SslStream};
use std::net::TcpStream;

mod client;
mod error;

// Type of connection used by the IMAP client
pub type Connection = Client<SslStream<TcpStream>>;

