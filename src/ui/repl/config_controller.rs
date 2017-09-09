use cuckoo::config::Config;
use ui::repl::{ReplUI, Command, CommandHandler};
use std::io::{Read, Write};
use cuckoo::ServiceBundle;
use super::utils;

pub struct ConfigController;

impl<I: Read, O: Write> CommandHandler<I, O> for ConfigController {
    fn handle(&self, ui: &mut ReplUI<I, O>, bundle: &mut ServiceBundle,
              command: &mut Command) {
        if command.ctype == "config" {
            self.handle_config_command(ui, command, &mut bundle.config);
            command.is_consumed = true;
        }
    }
}

impl ConfigController {

    fn handle_config_command<I: Read, O: Write>(
        &self,
        ui: &mut ReplUI<I, O>,
        command: &mut Command,
        configuration: &mut Config
    ) {
        let (sub_command, arguments) = utils::split_at_space(&command.argument_string);
        let sub_command = sub_command.as_str();

        match sub_command {
            "read" => self.handle_read_config(ui, &configuration, &arguments),
            "edit" => ui.writeln("config edit - Not yet implemented"),
            "" => ui.writeln("Please supply a sub-command. Valid sub-commands are {read | edit}"),
            _ => ui.writeln(&format!("Unknown sub-command '{}' for config", sub_command)),
        };

        command.is_consumed = true
    }

    fn handle_read_config<I: Read, O: Write>(&self, ui: &mut ReplUI<I, O>, configuration: &Config,
                                             arguments: &str) {
        let (sub_command, _) = utils::split_at_space(arguments.to_lowercase().as_str());
        let sub_command = sub_command.as_str();

        match sub_command {
            "" | "all" => ui.writeln("config read all - Not yet implemented"),
            "accounts" => {
                for acc in configuration.accounts.iter() {
                    ui.writeln(&format!("   Account: {}", acc.id));
                    ui.writeln(&format!("   Name: {}", acc.name));
                    ui.writeln(&format!("   Address: {}", acc.mail));
                    ui.writeln(&format!("   IMAP server: {}", acc.imap_server));
                    ui.writeln(&format!("   IMAP user: {}", acc.imap_user));
                    ui.writeln("");
                }
            }
            _ => ui.writeln(&format!("Unknown argument '{}'", sub_command)),
        }
    }
}