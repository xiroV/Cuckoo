use std::env;
use std::path;
extern crate config;

fn main() {
    let mut conf_file = path::PathBuf::new();
    let mut conf = config::Config::new();

    println!("Starting Cuckoo...");

    // Find and read config file
    match env::home_dir() { 
        Some(path) => { conf_file = path; },
        None => println!("Unable to find home directory. Config file not found")
    }

    conf_file.push(".config");
    conf_file.push("cuckoo");
    conf_file.push("config.toml");

    println!("Reading config file: {}", conf_file.display());
    
    // Get values from config file
    conf.merge(config::File::new(conf_file.to_str().unwrap(), config::FileFormat::Toml)).unwrap();

    // Loop through accounts
    for (account, values) in conf.get("accounts").unwrap().into_table().unwrap() { 
        println!("\nFetching mail for account {:?}", account);

        let account_info = values.into_table().unwrap();

        // TODO Fetch mails
        
        // Following print statements just to get some visuals
        println!("name: {:?}", account_info["name"].unwrap());
        println!("mail: {:?}", account_info["mail"]);
        println!("imap_server: {:?}", account_info["imap_server"]);
        println!("imap_user: {:?}", account_info["imap_user"]);
    }
}
