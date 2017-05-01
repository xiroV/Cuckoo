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
    
    // Set config default values
    conf.set_default("name", "John Doe");
    conf.set_default("maildir", "");
    
    // Get values from config file
    conf.merge(config::File::new(conf_file.to_str().unwrap(), config::FileFormat::Toml)).unwrap();

    // println!("{:?}", conf.get("name"));
}
