use frontend::simple_cli::{SimpleCli, messages, control};
use frontend::Runnerable;
use std::io;

impl Runnerable for SimpleCli {

	fn new() -> Self {
		let new_cli = SimpleCli { 
			cli_id: 0
		};
		return new_cli;
	}
	
	fn main_loop(self) {
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

			assert_eq!(buffer.pop(), Some('\n'), "Error broken line");
			
			let splitted: Vec<String> = buffer.split(' ').map(|s| s.to_string()).collect();
			
			if buffer == "quit" || buffer == "QUIT" {
				messages::replyln(messages::GOODBYE);
				break;
			}
			if splitted.len() > 0 {
				control::send_control(&splitted);
			}
		}
	}
}
