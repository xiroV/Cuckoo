use frontend::simple_cli::{SimpleCli, messages, control};
use config::{ConfigReader, Config};
use frontend::Runnerable;
use std::io;

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
			
			match stdin.read_line(&mut buffer) {
				Err(error) => {
					// FIXME find out how variadic arguments works 
					messages::replyln(&format!("Error: {}", error));
				},
				_ => {}
			};

			match buffer.pop() {
				None => {
					messages::replyln(messages::BROKEN_LINE);	
				},
				_ => {}
			};
			
			if buffer == "quit" || buffer == "QUIT" {
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
