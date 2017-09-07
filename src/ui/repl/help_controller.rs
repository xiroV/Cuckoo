use cuckoo::ServiceBundle;
use ui::repl::{ReplUI, Command, CommandHandler};
use std::io::{Read, Write};

pub struct HelpController;

impl<I: Read, O: Write> CommandHandler<I, O> for HelpController {
    fn handle(&self, ui: &mut ReplUI<I, O>, _bundle: &mut ServiceBundle, command: &mut Command) {
        command.is_consumed = true;
        match command.ctype.as_ref() {
         	"help" | "?" => handle_display_help(ui, command),
         	_ => command.is_consumed = false,
        }     
    }
}

fn handle_display_help<I: Read, O: Write>(ui: &mut ReplUI<I, O>, command: &mut Command) {
	ui.writeln("Valid commands are:");
    ui.writeln("  quit - quit this session");
    ui.writeln("  config [read|edit] - create or read configuration files");
}