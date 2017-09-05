use super::ReplUI;
use std::io::{Read, Write};

pub fn display_menu<I: Read, O: Write>(
    ui: &mut ReplUI<I, O>,
    message: &str,
    menu_items: Vec<&str>
) -> Option<usize> {
    loop {
        ui.writeln(message);
        for (i, item) in menu_items.iter().enumerate() {
            ui.writeln(&format!("  {}) {}", i + 1, item));
        }
        ui.write(">>> ");
        let result = ui.read_line();
        if result.is_none() { return None; };
        let result = result.unwrap().trim().to_string();
        let option = result.parse::<usize>();
        if option.is_ok() {
            let option = option.unwrap() - 1;
            if option < menu_items.len() {
                return Some(option);
            };
        }
    }
}