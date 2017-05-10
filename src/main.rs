mod config;
use config::ConfigReader;

fn main() {
    println!("Starting Cuckoo...");

    // Read config file
    let mut conf = config::Config::new();
    match conf.read() {
        None => { },
        Some(err) => println!("Error: {}", err),
    }
        
    // Print stuff from the config file (Just for visuals)
    for acc in conf.accounts {
        println!("\nAccount: {}", acc.id);
        println!("Name: {}", acc.name);
        println!("Address: {}", acc.mail);
        println!("IMAP server: {}", acc.imap_server);
        println!("IMAP user: {}", acc.imap_user);
    }
}
