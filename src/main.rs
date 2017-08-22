mod config;
mod frontend;
mod imap;
use frontend::Runnerable;
use frontend::simple_cli::SimpleCli;

fn main() {
    let cli = SimpleCli::new();
    cli.main_loop();
}
