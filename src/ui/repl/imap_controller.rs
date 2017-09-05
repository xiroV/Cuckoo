use ui::repl::{ReplUI, Command, CommandHandler};
use cuckoo::config::ConfigReader;
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
            command.is_consumed = true;
        }
    }
}

