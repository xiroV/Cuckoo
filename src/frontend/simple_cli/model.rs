use frontend::simple_cli::messages;
use config::{ConfigReader, Config};

struct ConfigModel {
	reader: Config
}
// TODO
impl ConfigModel {
	fn read(&mut self) {
		match self.reader.read() {
			None => { },
        	Some(err) => messages::replyln(&format!("Error: {}", err)),
		};
	}

	fn edit(&mut self) {

	}
}