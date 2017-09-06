use ui::repl::{ReplUI, Command, CommandHandler};
use cuckoo::ServiceBundle;
use std::io::{Read, Write};
use super::utils;
use super::menu;

pub struct ExitController;

impl<I: Read, O: Write> CommandHandler<I, O> for ExitController {
    fn handle(&self, ui: &mut ReplUI<I, O>, _bundle: &mut ServiceBundle,
              command: &mut Command) {
        if command.ctype == "quit" {
            let (sub_command, _) = utils::split_at_space(&command.argument_string);

            if sub_command == "-y" || sub_command == "--yes" {
                ui.should_run = false;
            } else {
                let option = menu::display_menu(
                    ui,
                    "Are you sure?",
                    vec!["Quit application", "Keep application open"]
                ).unwrap_or(0);

                ui.should_run = option == 1;
            }
            command.is_consumed = true;
        }
    }
}
