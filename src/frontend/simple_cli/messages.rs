
// "server" reply is indented with ">> "
pub fn replyln(st: &str) {
	println!(">> {}", st);
}

pub const WELCOME1: &str = "Welcome to the Cuckoo Mail Client Command Line Interpreter.";
pub const WELCOME2: &str = "Type 'quit' or 'QUIT' to exit, 'help' or 'HELP' to get a list of commands.";
pub const GOODBYE: &str = "Bye.";
pub const UNKNOWN_COMMAND: &str = "? Try 'help'.";

pub const BROKEN_LINE: &str = "Warning broken line";

pub const HELP_MSG: &str = "Valid commands are: \
	\n   quit - quit this session \
	\n   config [read|edit] - create or read configuration files";

pub const HELP_CONFIG: &str = "config: Read or edit the client configuration file";

pub const CONFIG_NO_ARGS: &str = "Missing arguments. Valid arguments are:\
	\n   read - reads the currently loaded configuration file\
	\n   edit - make changes to the currently loaded configuration file or create a new one if no file exists";

pub const IMAP_CONNECTION_SUCCESS: &str = "Connection established";
pub const IMAP_CONNECTION_FAILED: &str = "Failed to connect to the given mail server";
pub const IMAP_LOGIN_FAILED: &str = "Failed to login to mail server. Please check your credentials";
