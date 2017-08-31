use std::io::{Read, BufReader, BufWriter, BufRead, Write};
use std::option::Option;
use cuckoo::ServiceBundle;

pub mod help_controller;
pub mod config_controller;

#[derive(Debug)]
pub struct Command {
    // TODO This seems silly, there shouldn't be a need for three copies of the same string, but
    // apparently it isn't trivial to have &str point to internal field
    pub raw_command: String,
    pub ctype: String,
    pub argument_string: String,
    pub is_consumed: bool
}

impl Command {
    fn new(raw_command: String) -> Command {
        // Trim new lines and trailing whitespace
        let trimmed = raw_command.trim_right().to_string();
        let length = trimmed.len();
        let split_at = trimmed.chars()
            .enumerate()
            .position(|(_, c)| c == ' ')
            .unwrap_or(length);

        let ctype = (&trimmed[0..split_at]).to_string();
        let argument_string = if split_at == length {
            "".to_string()
        } else {
            (&trimmed[(split_at + 1)..length]).to_string()
        };

        Command {
            raw_command: trimmed,
            ctype: ctype,
            argument_string: argument_string,
            is_consumed: false
        }
    }
}

pub trait CommandHandler<I: Read, O: Write> {
    fn handle(&self, ui: &mut ReplUI<I, O>, services: &mut ServiceBundle, command: &mut Command);
}

pub struct Repl<I: Read, O: Write> {
    ui: ReplUI<I, O>,
    function_handlers: Vec<Box<CommandHandler<I, O>>>,
    service_bundle: ServiceBundle
}

impl<I: Read, O: Write> Repl<I, O> {
    pub fn new(ins: I, outs: O, bundle: ServiceBundle) -> Self {
        Repl {
            ui: ReplUI::new(ins, outs),
            function_handlers: vec![],
            service_bundle: bundle
        }
    }

    pub fn main_loop(&mut self) {
        loop {
            self.ui.display_prompt();
            match self.ui.read_command() {
                Some(mut command) => {
                    for handler in self.function_handlers.iter() {
                        handler.handle(&mut self.ui, &mut self.service_bundle, &mut command);
                    }
                }
                None => break
            }
        }
    }

    pub fn add_handler(&mut self, handler: Box<CommandHandler<I, O>>) {
        self.function_handlers.push(handler);
    }
}

pub struct ReplUI<I: Read, O: Write> {
    pub prompt: String,
    buffered_reader: BufReader<I>,
    buffered_writer: BufWriter<O>
}

impl<I: Read, O: Write> ReplUI<I, O> {
    fn new(ins: I, outs: O) -> Self {
        ReplUI {
            prompt: "$ ".to_string(),
            buffered_reader: BufReader::new(ins),
            buffered_writer: BufWriter::new(outs)
        }
    }

    fn read_command(&mut self) -> Option<Command> {
        let mut buffer = String::new();
        let read = self.buffered_reader.read_line(&mut buffer).unwrap();
        if read == 0 {
            None
        } else {
            Some(Command::new(buffer))
        }
    }

    fn read_line(&mut self) -> Option<String> {
        let mut buffer: String = String::new();
        let read: usize = self.buffered_reader.read_line(&mut buffer).unwrap_or(0);
        if read == 0 { None } else { Some(buffer) }
    }

    fn display_prompt(&mut self) {
        self.buffered_writer.write(&self.prompt.as_bytes()).unwrap();
        self.buffered_writer.flush().unwrap();
    }

    pub fn write(&mut self, message: &str) {
        self.buffered_writer.write(message.as_bytes()).unwrap();
    }

    pub fn writeln(&mut self, message: &str) {
        self.write(message);
        self.write("\n");
    }
}

