use std::io::{Read, BufReader, BufWriter, BufRead, Write};
use std::option::Option;
use cuckoo::ServiceBundle;
use std::clone::Clone;
use std::mem;

pub mod help_controller;
pub mod config_controller;
pub mod exit_controller;
pub mod imap_controller;
mod utils;
mod menu;

#[derive(Debug)]
pub struct Command {
    pub ctype: String,
    pub argument_string: String,
    pub is_consumed: bool
}

impl Command {
    fn new(raw_command: String) -> Command {
        // Trim new lines and trailing whitespace
        let (ctype, argument_string) = utils::split_at_space(&raw_command.trim());

        Command {
            ctype: ctype.to_lowercase(),
            argument_string: argument_string,
            is_consumed: false
        }
    }
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Command {
            ctype: self.ctype.clone(),
            argument_string: self.argument_string.clone(),
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

fn preproces_command<I: Read, O: Write>(ui: &mut ReplUI<I, O>, command: &mut Command) {
    match command.ctype.as_ref() {
        "!!" => {
            match ui.last_command {
                Some(ref mut last_cmd) => {
                    if last_cmd.argument_string.trim().is_empty() {
                        mem::swap(&mut last_cmd.argument_string, &mut command.argument_string);
                    } else {
                        last_cmd.argument_string = format!("{} {}", last_cmd.argument_string.trim(), command.argument_string);
                    }
                    mem::swap(command, last_cmd);
                },
                None => {
                    command.is_consumed = true;
                    ui.writeln("No previous command found.");
                },
            }
        },
        _ => {},
    }
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
        self.ui.writeln("Welcome to the Cuckoo Mail Client Command Line Interpreter.");
        self.ui.writeln("Type 'quit' to exit, 'help' to get a list of commands.");
        while self.ui.should_run {
            self.ui.display_prompt();
            match self.ui.read_command() {
                Some(mut command) => {

                    preproces_command(&mut self.ui, &mut command);

                    for handler in self.function_handlers.iter() {
                        handler.handle(&mut self.ui, &mut self.service_bundle, &mut command);
                    }
                    
                    if !command.is_consumed {
                        self.ui.writeln("Command not recognized. Try 'help' or '?'.")
                    } 
                    if command.ctype != "!!" {
                        self.ui.last_command = Some(command.clone());
                    }
                }
                None => break
            }
        }
        self.ui.writeln("Bye.")
    }

    pub fn add_handler(&mut self, handler: Box<CommandHandler<I, O>>) {
        self.function_handlers.push(handler);
    }
}

pub struct ReplUI<I: Read, O: Write> {
    pub prompt: String,
    pub should_run: bool,
    pub last_command: Option<Command>,
    buffered_reader: BufReader<I>,
    buffered_writer: BufWriter<O>
}

impl<I: Read, O: Write> ReplUI<I, O> {
    fn new(ins: I, outs: O) -> Self {
        ReplUI {
            prompt: ">> ".to_string(),
            should_run: true,
            buffered_reader: BufReader::new(ins),
            buffered_writer: BufWriter::new(outs),
            last_command: None
        }
    }

    fn read_command(&mut self) -> Option<Command> {
        self.read_line().map(|it| Command::new(it))
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

    #[allow(dead_code)]
    pub fn write(&mut self, message: &str) {
        self.buffered_writer.write(message.as_bytes()).unwrap();
        self.buffered_writer.flush().unwrap();
    }

    #[allow(dead_code)]
    pub fn writeln(&mut self, message: &str) {
        self.buffered_writer.write(message.as_bytes()).unwrap();
        self.buffered_writer.write("\n".as_bytes()).unwrap();
        self.buffered_writer.flush().unwrap();
    }
}

