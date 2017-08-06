
// "server" reply is indented with ">> "
pub fn replyln(st: &str) {
	println!(">> {}", st);
}

pub const WELCOME1: &str = "Welcome to the Cuckoo Mail Client Command Line Interpreter.";
pub const WELCOME2: &str = "Type 'quit' or 'QUIT' to exit, 'help' or 'HELP' to get a list of commands.";
pub const GOODBYE: &str = "Bye.";
pub const UNKNOWN_COMMAND: &str = "? Try 'help'.";

pub const HELP_MSG: &str = "Valid commands are: \
	\n   quit - quit this session \
	\n   config [read|edit] - create or read configuration files";

pub const CONFIG_NO_ARGS: &str = "Missing arguments. Valid arguments are:\
	\n   read - reads the currently loaded configuration file\
	\n   edit - make changes to the currently loaded configuration file or create a new one if no file exists";
