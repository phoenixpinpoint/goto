use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;
use std::env;
use std::collections::HashMap;

fn get_home_dir() -> String {
    // Try HOME first (Unix/Linux/macOS), then USERPROFILE (Windows)
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .expect("Neither HOME nor USERPROFILE environment variable is set")
}

pub fn load_shortcuts() -> HashMap<String, String> {
    let home = get_home_dir();
    let path = PathBuf::from(home.clone()).join(".shortcuts");
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
                    let home = get_home_dir();
                    value = value.replacen("~", &home, 1);
                } else if value == "~" {
                    value = get_home_dir();
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

fn print_usage() {
    println!("goto - Navigate to saved directory shortcuts\n");
    println!("Usage:");
    println!("  goto <shortcut>           Navigate to a saved shortcut");
    println!("  goto -a <name>            Add current directory as a shortcut");
    println!("  goto -a <name>=<path>     Add specified path as a shortcut");
    println!("  goto -l                   List all shortcuts");
    println!("  goto -h                   Show this help message");
}

fn process_list_shortcuts() {
   let shortcuts = load_shortcuts();
    for (key, value) in shortcuts.iter() {
        println!("{} = {}", key, value);
    } 
}

fn process_add_shortcut(arg: &str) {
    let (name, path) = if arg.contains('=') {
        // Parse shortcut=path format
        let parts: Vec<&str> = arg.splitn(2, '=').collect();
        if parts.len() != 2 {
            eprintln!("Error: Invalid shortcut definition. Use the format <name>=<path>");
            return;
        }
        (parts[0].trim().to_string(), parts[1].trim().to_string())
    } else {
        // Use current directory
        let current_dir = env::current_dir()
            .expect("Unable to get current directory")
            .to_str()
            .expect("Invalid path")
            .to_string();
        (arg.to_string(), current_dir)
    };

    let mut shortcuts = load_shortcuts();
    shortcuts.insert(name.clone(), path.clone());

    // Save back to file
    let home = get_home_dir();
    let file_path = PathBuf::from(home).join(".shortcuts");
    let mut file = fs::File::create(file_path).expect("Unable to open shortcuts file for writing");

    for (key, value) in shortcuts.iter() {
        let line = format!("{}={}\n", key, value);
        file.write_all(line.as_bytes()).expect("Unable to write to shortcuts file");
    }

    println!("Shortcut '{}' added for path '{}'", name, path);
}

fn process_goto(shortcut: &String, shortcuts: &HashMap<String, String>) -> bool{
    if let Some(path) = shortcuts.get(shortcut) {
        // Print with special prefix so shell function can detect it
        println!("GOTO:{}", path);
        return true; // Return true if shortcut found
    } else {
        eprintln!("Shortcut not found: {}", shortcut);
    }

    false // Return false if shortcut not found
}

fn process_args(args: &mut Vec<String>) {
    args.remove(0); // Remove the program name
    
    if args.len() == 0 {
        //If no futher args exist print usage
        print_usage();
    }

    let mut iterator = args.iter();

    while let Some(arg) = iterator.next() {
        match arg.as_str() {
            "-h" => {
                print_usage();
            }

            "-l" => {
                process_list_shortcuts();
            }

            "-a" => {
                if let Some(name) = iterator.next() {
                    process_add_shortcut(name);
                } else {
                    eprintln!("Error: No shortcut name provided for -a option");
                }
            }
            _ => {
                // If the argument is a shortcut, execute goto
                if args.len() == 1 {
                    if !process_goto(arg, &load_shortcuts()) {
                        // Only print usage if shortcut not found
                        print_usage();
                    }
                } else {
                    // Multiple arguments that don't match flags
                    print_usage();
                }
            }
        }
    }
    
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    //dbg!(args);
    process_args(&mut args);
    //dbg!(commands);

}
