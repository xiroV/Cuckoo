use frontend::simple_cli::messages;

// dispatches the functions according to the first inputted argument
pub fn send_control(tokens: &[String]) {
	match tokens[0].to_lowercase().as_str() {
		"config" => handle_config(&tokens[1..]),
		"help" => handle_help(&tokens[1..]),
		_ => messages::replyln(messages::UNKNOWN_COMMAND)
	}
}

// "top"-level handler for the configuration  
fn handle_config(arguments: &[String]) {
	if arguments.len() == 0 {
		messages::replyln(messages::CONFIG_NO_ARGS);
	} else {
		match arguments[0].as_str() {
			"read" => messages::replyln("Reading config file..."), // put in your function handling here
			"edit" => messages::replyln("Preparing to edit file..."),
			_ => messages::replyln("buuh")
		}
	}
}

// show help. Addition argument for specifying what topic 
// help should be shown for could be implemeted
fn handle_help(arguments: &[String]) {
	if arguments.len() == 0 {
		messages::replyln(messages::HELP_MSG);	
	}
}
