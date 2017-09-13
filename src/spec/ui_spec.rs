use ui::repl::{Repl, help_controller, Command, ReplUI};
use cuckoo;
use cuckoo::config::{Config, ConfigReader};
use std::io::{Cursor};


fn get_repl_instance(services: cuckoo::ServiceBundle) -> Repl<Cursor<Vec<u8>>, Cursor<Vec<u8>>> {
    Repl::new(Cursor::new(Vec::new()), Cursor::new(Vec::new()), services)
}


#[test]
fn repl_can_be_initialized() {

    let service_bundle = cuckoo::ServiceBundle {
        config: Config::new()
    };

    let mut repl = get_repl_instance(service_bundle);

    assert!(repl.get_handlers().is_empty());
    assert!(repl.get_service_bundle().config.accounts.is_empty());
}

#[test]
fn repl_can_add_handler() {

    let service_bundle = cuckoo::ServiceBundle {
        config: Config::new()
    };

    let mut repl = get_repl_instance(service_bundle);

    assert!(repl.get_handlers().is_empty());

    let help_handler = Box::new(help_controller::HelpController {});

    repl.add_handler(help_handler);

    assert!(!repl.get_handlers().is_empty());
    assert!(repl.get_handlers().len() == 1);
}

#[test]
fn command_can_create_new_command() {

    let raw_command = "config read";
    let command = Command::new(raw_command.to_string());

    assert!(command.ctype.len() > 0);
    assert!(command.ctype == "config");
    assert!(command.argument_string.len() > 0);
    assert!(command.argument_string == "read");
}

#[test]
fn replui_can_be_initialized() {
    let reader = Cursor::new(vec![]);
    let writer = Cursor::new(vec![]);

    let repl_ui = ReplUI::new(reader, writer);

    assert!(repl_ui.prompt == ">> ");
    assert!(repl_ui.should_run == true);
    assert!(repl_ui.last_command.is_none());
}

#[test]
fn replui_can_read() {
    let reader = Cursor::new(b"new mail");
    let writer = Cursor::new(vec![]);

    let mut repl_ui = ReplUI::new(reader, writer);

    let new_line = repl_ui.read_line().unwrap();
    assert!(new_line.len() > 0);
    assert!(new_line == "new mail");
}

#[test]
fn replui_can_read_command() {
    let reader = Cursor::new(b"config edit");
    let writer = Cursor::new(vec![]);

    let mut repl_ui = ReplUI::new(reader, writer);

    let new_command = repl_ui.read_command().unwrap();
    assert!(new_command.ctype.len() > 0);
    assert!(new_command.ctype == "config");
    assert!(new_command.argument_string.len() > 0);
    assert!(new_command.argument_string == "edit");
}

#[test]
fn replui_can_write() {
    let reader = Cursor::new(vec![]);
    let writer = Cursor::new(vec![]);
    let message = "new mail";
    {
        let inner: &Vec<u8> = writer.get_ref();
        assert!(inner.is_empty());
    }

    let mut repl_ui = ReplUI::new(reader, writer);
    repl_ui.write(message);

    let written_content = repl_ui.get_bufwriter().get_ref().get_ref();

    assert!(written_content.len() > 0);
    assert!(written_content == b"new mail");
}

#[test]
fn replui_can_writeln() {
    let reader = Cursor::new(vec![]);
    let writer = Cursor::new(vec![]);
    let message = "new mail";
    {
        let inner: &Vec<u8> = writer.get_ref();
        assert!(inner.is_empty());
    }

    let mut repl_ui = ReplUI::new(reader, writer);
    repl_ui.writeln(message);

    let written_content = repl_ui.get_bufwriter().get_ref().get_ref();

    assert!(written_content.len() > 0);
    assert!(written_content == b"new mail\n");
}

#[test]
fn replui_can_show_prompt() {
    let reader = Cursor::new(vec![]);
    let writer = Cursor::new(vec![]);
    let prompt = b">> ";
    {
        let inner: &Vec<u8> = writer.get_ref();
        assert!(inner.is_empty());
    }

    let mut repl_ui = ReplUI::new(reader, writer);
    repl_ui.display_prompt();

    let written_content = repl_ui.get_bufwriter().get_ref().get_ref();

    assert!(written_content.len() > 0);
    assert!(written_content == prompt);
}
