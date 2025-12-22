use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;
use std::env;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Cmds {
    ListShortcuts,
    AddShortcut,
    DeleteShortcut,
    Goto,
    Usage,
}

#[derive(Debug)]
pub struct Cmd {
    pub cmd_type: Cmds,
    pub args: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Commands {
    pub raw: String,
    pub cmds: Vec<Cmd>,
} 

pub fn load_shortcuts() -> HashMap<String, String> {
    let home = env::var("HOME").expect("HOME environment variable not set");
    let path = PathBuf::from(home).join(".shortcuts");
    let mut shortcuts = HashMap::new();
    if path.exists() {
        let mut file = fs::File::open(path).expect("Unable to open shortcuts file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read shortcuts file");
        // Process the contents as needed
        for line in contents.lines() {
            //println!("{}", line);
            let line_parts: Vec<&str> = line.splitn(2, '=').collect();
            if line_parts.len() == 2 {
                let key = line_parts[0].trim().to_string();
                let mut value = line_parts[1].trim().to_string();
                
                // Expand ~ to home directory
                if value.starts_with("~/") {
                    let home = env::var("HOME").expect("HOME environment variable not set");
                    value = value.replacen("~", &home, 1);
                } else if value == "~" {
                    value = env::var("HOME").expect("HOME environment variable not set");
                }
                
                //println!("Key: {}, Value: {}", key, value);
                shortcuts.insert(key, value);
            }
        }
    } else {
        // Create the file if it doesn't exist
        fs::File::create(path).expect("Unable to create shortcuts file");
    }
    shortcuts
}