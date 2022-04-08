extern crate dirs;
extern crate subprocess;

use chrono::{Datelike, Utc};
use dirs::config_dir;
use dirs::home_dir;
use std::fs;
use std::io::ErrorKind;
use std::{
    env,
    io::{Read, Write},
};
use toml::Value;

fn main() {
    // Read args (except 1)
    let args: Vec<String> = env::args().skip(1).collect();

    let configs = read_config();

    // Handle empty configs
    let notes_dir = match configs.get("directory_name") {
        Some(directory_name) => directory_name.as_str().unwrap(),
        None => "notes", // Use ~/notes if notes directory name is not set
    };

    let editor = match configs.get("editor") {
        Some(editor) => editor.as_str().unwrap().to_string(),
        None => get_system_editor(), // Use system editor if there is no editor on config
    };

    let note_filetype = match configs.get("filetype") {
        Some(filetype) => filetype.as_str().unwrap(),
        None => "", // No file extension if there is no filetype on config
    };

    // Create files
    let now = Utc::now();
    let year = now.year();
    let month = format!("{:0>#2}", now.month());
    let day = format!("{:0>#2}", now.day());
    let home_path = match home_dir() {
        Some(home_dir) => home_dir,
        None => panic!("Couldn't find home directory."),
    };

    let dir_path = format!(
        "{}/{}/{}/{}",
        home_path.to_str().unwrap(),
        notes_dir,
        year,
        month
    );
    let file_path = format!("{}/{}{}", dir_path, day, note_filetype);

    fs::create_dir_all(dir_path).expect("Error creating directories.");

    if args.len() > 0 {
        // Append args to end of file if there are any args
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
        // Open editor if there are no args
        subprocess::Exec::cmd(format!("{}", editor))
            .arg(file_path)
            .join()
            .unwrap();
    }
}

fn read_config() -> Value {
    let mut config_toml = String::new();
    let config_dir = match config_dir() {
        Some(dir) => dir,
        None => panic!("Couldn't find system configuration directory.")
    };

    let config_file_path = format!(
        "{}/notes/config.toml",
        config_dir.to_str().unwrap()
    );

    // Open config file
    let config_file = fs::OpenOptions::new()
        .read(true)
        .open(&config_file_path);

    // Handle config file doensn't existing
    let mut config_file = match config_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                fs::create_dir_all(format!("{}/notes", config_dir.to_str().unwrap())).expect("Error creating config file");
                fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&config_file_path)
                        .unwrap_or_else(|err| panic!("Error opening/creating configuration file: [{:?}]", err))
            },
            other_error => panic!("{}", other_error)
        },
    };

    config_file
        .read_to_string(&mut config_toml)
        .unwrap_or_else(|err| panic!("Error while reading configuration file: [{:?}],", err));

    config_toml.parse::<Value>().unwrap()
}

fn get_system_editor() -> String {
    env::var("EDITOR").expect("$EDITOR is not set")
}
