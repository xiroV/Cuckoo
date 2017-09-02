use cuckoo::ServiceBundle;
use ui::repl::{ReplUI, Command, CommandHandler};
use std::io::{Read, Write};

pub struct HelpController;

impl<I: Read, O: Write> CommandHandler<I, O> for HelpController {
    fn handle(&self, ui: &mut ReplUI<I, O>, _bundle: &mut ServiceBundle, command: &mut Command) {
        if command.ctype == "help" || command.ctype == "?" {
            ui.writeln("Valid commands are:");
            ui.writeln("  quit - quit this session");
            ui.writeln("  config [read|edit] - create or read configuration files");
            command.is_consumed = true;
        }
    }
}
