use frontend::simple_cli::messages;
use config::{ConfigReader, Config, ConfigErrorType, ConfigError};

struct ConfigModel {
	reader: Config
}
// TODO
impl ConfigModel {
	fn read(&mut self) {
		match self.reader.read() {
			Ok(_) => { },
        	Err(err) => match err.err_type {
                ConfigErrorType::MissingField => messages::replyln(&format!("Error: Missing field {:?}", err.field)),
                ConfigErrorType::FileNotFound => messages::replyln("Error: Config file not found"),
            }
		};
	}

	fn edit(&mut self) {

	}
}
