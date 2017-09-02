use cuckoo::config::Config;
use ui::repl::{ReplUI, Command, CommandHandler};
use std::io::{Read, Write};
use cuckoo::ServiceBundle;

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
    pub fn new() -> Self {
        ConfigController {}
    }

    fn handle_config_command<I: Read, O: Write>(
        &self, ui: &mut ReplUI<I, O>, _command: &mut Command, configuration: &mut Config
    ) {
        ui.writeln(&format!("You have {} accounts", configuration.accounts.len()));
    }
}