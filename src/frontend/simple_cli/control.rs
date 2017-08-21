use frontend::simple_cli::{messages, model, SimpleCli};
use config::{ConfigReader, Config};
use imap::{IMAPClient, IMAP, IMAPError};
use std::io;

// dispatches the functions according to the first inputted argument
pub fn send_control(client:&mut SimpleCli, tokens: &[String]) {
	match tokens[0].to_lowercase().as_str() {
		"config" => handle_config(&tokens[1..]),
		"help" | "?" => handle_help(&tokens[1..]),
		"connect" => handle_connect(client, &tokens[1..]),
		"mail" => handle_mail(client, &tokens[1..]),
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
	} else if arguments[0].to_lowercase() == "accounts" {
		let mut conf = Config::new();
	    match conf.read() {
	        None => { },
	        Some(err) => println!("Error: {}", err),
	    }
	        
	    // Print stuff from the config file (Just for visuals)
	    for acc in conf.accounts {
	        messages::replyln(&format!("Account: {}", acc.id));
	        println!("   Name: {}", match acc.name {
	            Some(name) => name,
	            None => String::new() 
	        });
	        println!("   Address: {}", match acc.mail {
	            Some(mail) => mail,
	            None => String::new()
	        });
	        println!("   IMAP server: {}", match acc.imap_server {
	            Some(imap_server) => imap_server,
	            None => String::new()
	        });
	        println!("   IMAP user: {}", match acc.imap_user {
	            Some(imap_user) => imap_user,
	            None => String::new()
	        });
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

// Handle IMAP Connection
fn handle_connect(client:&mut SimpleCli, arguments: &[String]) {
    if arguments.len() == 1 {
        // Fetch login information from the config file
        let mut conf = Config::new();

        match conf.read() {
            None => { },
            Some(err) => println!("{}", err)
        }


        // FIXME This is a mess
        match conf.get_account(&arguments[0]) {
            Some(acc) => {
                match acc.imap_server {
                    Some(imap_server) => {
                        match acc.imap_user {
                            Some(imap_user) => {
                                let stdin = io::stdin();
                                let mut password = String::new();

                                println!("Password:");

                                match stdin.read_line(&mut password) {
                                    Err(err) => {
                                        messages::replyln(&format!("Error: {}", err));
                                    },
                                    _ => {}
                                };

                                println!("Connecting..");

                                match IMAP::connect(&imap_server, &imap_user, &password) {
                                    Ok(conn) => {
                                        client.imap_connection = Some(conn);
                                        messages::replyln(messages::IMAP_CONNECTION_SUCCESS)
                                    },
                                    Err(e) => match e {
                                        IMAPError::Connection => messages::replyln(messages::IMAP_CONNECTION_FAILED),
                                        IMAPError::Login => messages::replyln(messages::IMAP_LOGIN_FAILED),
                                    }
                                };
                            },
                            None => {
                                messages::replyln(messages::ACCOUNT_IMAP_USER_NOT_FOUND);
                            }
                        };
                    },
                    None => {
                        messages::replyln(messages::ACCOUNT_IMAP_SERVER_NOT_FOUND);
                    }
                };
            },
            None => {
                messages::replyln(messages::ACCOUNT_NOT_FOUND);
            }
        };
    } else {
        messages::replyln("Not enough arguments");
    }
}

fn handle_mail(client: &mut SimpleCli, arguments: &[String]) {
    println!("Not yet implemented");
    /*match client.imap_connection {
        Some(ref mut conn) => {
            match conn.fetch(&arguments[0]) {
                Ok(result) => {
                    for line in result {
                        println!("{}", line);
                    }
                },
                Err(e) => messages::replyln(messages::IMAP_NO_RESULT),
            };
        },
        None => {
            messages::replyln(messages::IMAP_NO_CONNECTION);
        }
    };*/

}
