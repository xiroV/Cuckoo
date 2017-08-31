use cuckoo::ServiceBundle;
use ui::repl::{ReplUI, Command, CommandHandler};
use std::io::{Read, Write};

pub struct HelpController;

impl<I: Read, O: Write> CommandHandler<I, O> for HelpController {
    fn handle(&self, ui: &mut ReplUI<I, O>, bundle: &mut ServiceBundle, command: &mut Command) {
        println!("Got command: {:?}", &command);
        if !command.is_consumed && command.ctype == "help" {
            ui.writeln("This is a help message");
            command.is_consumed = true;
        }
    }
}
