use frontend::simple_cli::{messages, model};
use config::{ConfigReader, Config, ConfigError, ConfigErrorType};

// dispatches the functions according to the first inputted argument
pub fn send_control(tokens: &[String]) {
	match tokens[0].to_lowercase().as_str() {
		"config" => handle_config(&tokens[1..]),
		"help" | "?" => handle_help(&tokens[1..]),
		_ => messages::replyln(messages::UNKNOWN_COMMAND)
	}
}

// "top"-level handler for the configuration  
fn handle_config(arguments: &[String]) {
	if arguments.len() == 0 {
		messages::replyln(messages::CONFIG_NO_ARGS);
	} else {
		match arguments[0].as_str() {
			"read" => read_config(&arguments[1..]), //messages::replyln("Reading config file..."), // put in your function handling here
			"edit" => messages::replyln("Preparing to edit file..."),
			_ => messages::replyln("buuh")
		}
	}
}

fn read_config(arguments: &[String]) {

	if arguments.len() == 0 || arguments[0].to_lowercase() == "all" {
 		// read everything?
        println!("Not yet implemented");
	} else if arguments[0].to_lowercase() == "accounts" {
		let mut conf = Config::init();
	    match conf.read() {
	        Ok(_) => { },
	        Err(err) => match err.err_type {
                ConfigErrorType::MissingField => println!("Missing a field for one of your accounts in the config file: {:?}", err.field),
                ConfigErrorType::FileNotFound => println!("Your config file was not found"),
            }
	    }
	        
	    // Print stuff from the config file (Just for visuals)
	    for acc in conf.accounts {
	        messages::replyln(&format!("Account: {}", acc.id));
	        println!("   Name: {}", acc.name);
	        println!("   Address: {}", acc.mail);
	        println!("   IMAP server: {}", acc.imap_server);
	        println!("   IMAP user: {}", acc.imap_user);
	    }	
	}
}

// show help. Addition argument for specifying what topic 
// help should be shown for could be implemeted
fn handle_help(arguments: &[String]) {
	if arguments.len() == 0 {
		messages::replyln(messages::HELP_MSG);	
	} else {
		match arguments[0].as_str() {
			"config" | "configuration" => {
				messages::replyln(messages::HELP_CONFIG);
			},
			x => messages::replyln(&format!("No help for topic {:?}", x)),
		}
	}
}
