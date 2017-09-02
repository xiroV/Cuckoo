use cuckoo::ServiceBundle;
use ui::repl::{ReplUI, Command, CommandHandler};
use ui::repl::menu::display_menu;
use std::io::{Read, Write};

pub struct HelpController;

impl<I: Read, O: Write> CommandHandler<I, O> for HelpController {
    fn handle(&self, ui: &mut ReplUI<I, O>, _bundle: &mut ServiceBundle, command: &mut Command) {
        println!("Got command: {:?}", &command);
        if !command.is_consumed && command.ctype == "help" {
            ui.writeln("This is a help message");

            let opt = display_menu(
                ui,
                "Select an item",
                vec!("Option 1", "Option 2", "Or something else")
            ).unwrap();
            ui.writeln(&format!(
                "You chose {}", opt
            ));
            command.is_consumed = true;
        }
    }
}
