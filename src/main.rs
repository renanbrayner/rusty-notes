extern crate dirs;
extern crate subprocess;

use std::{env, io::{Read, Write}};
use fs::File;
use toml::Value;
use std::fs;
use chrono::{Datelike, Utc};
use dirs::home_dir;
use dirs::config_dir;

fn main() {
    // Read args (except 1)
    let args: Vec<String> = env::args().skip(1).collect();

    // Config files
    let configs = read_config();
    let editor = configs["editor"].as_str().unwrap();
    let journal_dir = configs["directory_name"].as_str().unwrap();
    
    // Create files
    let now = Utc::now();
    let year = now.year();
    let month = format!("{:0>#2}", now.month());
    let day = format!("{:0>#2}", now.day());
    let home_path = home_dir().unwrap();

    let dir_path = format!("{}/{}/{}/{}",home_path.to_str().unwrap(), journal_dir, year, month);
    let file_path = format!("{}/{}.txt",dir_path , day);

    fs::create_dir_all(dir_path).expect("Error creating directories.");

    if args.len() > 0 {
        // Append args to end of file
        let text = args.join(" ");
        let mut file = fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(file_path)
            .expect("Error creating/reading journal file.");

        file.write(text.as_bytes()).expect("Error writing to file");
        file.write(b"\n\n").expect("Error writing to file");
    } else {
        // Open editor
        subprocess::Exec::cmd(format!("{}", editor)).arg(file_path).join().unwrap();
    }
}

fn read_config() -> Value {
    let mut config_toml = String::new();
    let config_dir = config_dir().unwrap();
    let mut config_file = match File::open(&format!("{}/notes/config.toml", config_dir.to_str().unwrap())) {
        Ok(config_file) => config_file,
        Err(_) => {
            panic!("Could not found config file");
        }
    };

    config_file.read_to_string(&mut config_toml)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}],", err));

    config_toml.parse::<Value>().unwrap()
}
