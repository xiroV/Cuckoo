mod config;
use config::ConfigReader;

fn main() {
    println!("Starting Cuckoo...");

    // Read config file
    let mut conf = config::Config::new();
    conf.read();
        
    // Print stuff from the config file (Just for visuals)
    for acc in conf.accounts {
        println!("\nAccount: {}", acc.acc);
        println!("Name: {}", acc.name);
        println!("Address: {}", acc.mail);
        println!("IMAP server: {}", acc.imap_server);
        println!("IMAP user: {}", acc.imap_user);
    }
}
