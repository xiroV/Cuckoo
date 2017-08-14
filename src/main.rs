mod config;
mod frontend;
mod imap;
use frontend::Runnerable;
use frontend::simple_cli::SimpleCli;
//use config::ConfigReader;

fn main() {
    let cli = SimpleCli::new();

    cli.main_loop();
    //println!("Starting Cuckoo...");

    // Read config file
    /*
    let mut conf = config::Config::new();
    match conf.read() {
        None => { },
        Some(err) => println!("Error: {}", err),
    }
        
    // Print stuff from the config file (Just for visuals)
    for acc in conf.accounts {
        println!("\nAccount: {}", acc.id);
        println!("Name: {}", match acc.name {
            Some(name) => name,
            None => String::new() 
        });
        println!("Address: {}", match acc.mail {
            Some(mail) => mail,
            None => String::new()
        });
        println!("IMAP server: {}", match acc.imap_server {
            Some(imap_server) => imap_server,
            None => String::new()
        });
        println!("IMAP user: {}", match acc.imap_user {
            Some(imap_user) => imap_user,
            None => String::new()
        });
    }
    */

}
