mod ui;
mod cuckoo;

use ui::repl::Repl;
use cuckoo::config::{ConfigReader, Config};

fn main() {
    // TODO Names of the REPL are currently bad. Come up with something better than this.

    // Initialize services.
    // Move this into its own thing when this gets too big
    let mut config = Config::new();
    config.read().unwrap();

    let service_bundle = cuckoo::ServiceBundle {
        config: config
    };

    // The REPL can read and write to any source implementing Read or Write. For example we
    // can read from a predefined "script" and write to a vector with the following:

    //let reader: &[u8] = b"Testing, Testing, Testing\n2\n3\n4";
    //let out = Vec::new();

    let reader = std::io::stdin();
    let out = std::io::stdout();
    let mut repl = Repl::new(reader, out, service_bundle);

    {
        // Create and register command handlers.
        // Move this into its own thing when this gets too big
        let help = ui::repl::help_controller::HelpController {};
        repl.add_handler(Box::new(help));

        let config = ui::repl::config_controller::ConfigController::new();
        repl.add_handler(Box::new(config));
    }

    repl.main_loop();
}

