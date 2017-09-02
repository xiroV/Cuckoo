use ui::repl::{ReplUI, Command, CommandHandler};
use cuckoo::ServiceBundle;
use std::io::{Read, Write};
use super::utils;
use super::menu;
use cuckoo::config::Config;

pub struct IMAPController;

impl<I: Read, O: Write> CommandHandler<I, O> for IMAPController {
    fn handle(&self, ui: &mut ReplUI<I, O>, _bundle: &mut ServiceBundle,
              command: &mut Command) {
        if command.ctype == "imap" {
            let (sub_command, arguments) = utils::split_at_space(command.argument_string);
            let sub_command = sub_command.as_str();
            match sub_command {
                "connect" => {
                    handle_connect_to_account()
                }
            }
            command.is_consumed = true;
        }
    }
}

pub fn handle_connect_to_account<I: Read, O: Write>(ui: &mut ReplUI<I, O>, arguments: &str,
                                                    config: &Config) {
    let (account_name, _) = utils::split_at_space(arguments);
    let account = config.get_account(account_name);
    if account.is_none() {
        ui.writeln(&format!("Could not find account with name '{}'", account_name));
        return
    }

    let account = account.unwrap();
}
