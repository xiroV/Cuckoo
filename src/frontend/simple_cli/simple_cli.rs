use frontend::simple_cli::{SimpleCli, messages, control};
use config::{ConfigReader, Config};
use frontend::Runnerable;
use std::io;
use imap::IMAPClient;

impl Runnerable for SimpleCli {

	fn new() -> Self {
		SimpleCli {
            imap_connection: None, 
        }
	}
	
	fn main_loop(&mut self) {
		let stdin = io::stdin();

		messages::replyln(messages::WELCOME1);
		messages::replyln(messages::WELCOME2);
		loop {
			let mut buffer = String::new();
			print!("{}", messages::PROMPT);
			io::Write::flush(&mut io::stdout()).expect("Error flush failure");
			match stdin.read_line(&mut buffer) {
				Err(error) => {
					// FIXME find out how variadic arguments works 
					messages::replyln(&format!("Error: {}", error));
				},
				Ok(x) => {}
			};

			match buffer.pop() {
				None => {
					messages::replyln(messages::BROKEN_LINE);	
				},
				_ => {}
			};
			
			if buffer == "quit" || buffer == "QUIT" {
                match self.imap_connection {
                    // Close the connection, if exists
                    Some(ref mut conn) => {
                        messages::replyln(messages::DISCONNECTING);
                        conn.disconnect()
                    }
                    None => { }
                }
				messages::replyln(messages::GOODBYE);
				break;
			}

			let splitted: Vec<String> = buffer.split(' ').map(|s| s.to_string()).collect();

			if splitted.len() > 0 && splitted[0].len() > 0 {
				control::send_control(self, &splitted);
			}
		}
	}
}
