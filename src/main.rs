use std::env;
use std::collections::HashMap;

pub mod commands;
use commands::*;


fn print_usage() {
    println!("goto - Navigate to saved directory shortcuts\n");
    println!("Usage:");
    println!("  goto <shortcut>                 Navigate to a saved shortcut");
    println!("  goto -a <shortcut>=<path>       Add current directory as a shortcut");
    println!("  goto -l                         List all shortcuts");
    println!("  goto -h                         Show this help message");
}

fn process_list_shortcuts() {
   let shortcuts = load_shortcuts();
    for (key, value) in shortcuts.iter() {
        println!("{} = {}", key, value);
    } 
}

fn process_add_shortcut(args: &Vec<String>) {
    if args.len() != 1 {
        eprintln!("Error: Invalid number of arguments for adding shortcut");
        return;
    } else {
        let shortcut_def = &args[0];
        let parts: Vec<&str> = shortcut_def.splitn(2, '=').collect();
        if parts.len() != 2 {
            eprintln!("Error: Invalid shortcut definition. Use the format <shortcut>=<path>");
            return;
        }
        let shortcut = parts[0].trim();
        let path = parts[1].trim();

        let mut shortcuts = load_shortcuts();
        shortcuts.insert(shortcut.to_string(), path.to_string());

        // Save back to file
        let home = env::var("HOME").expect("HOME environment variable not set");
        let file_path = std::path::PathBuf::from(home).join(".shortcuts");
        let mut file = std::fs::File::create(file_path).expect("Unable to open shortcuts file for writing");

        for (key, value) in shortcuts.iter() {
            let line = format!("{}={}\n", key, value);
            use std::io::Write;
            file.write_all(line.as_bytes()).expect("Unable to write to shortcuts file");
        }

        println!("Shortcut '{}' added for path '{}'", shortcut, path); 
    }
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

fn process_args(args: &mut Vec<String>) -> Option<Commands> {
    args.remove(0); // Remove the program name

    let args_str = &args.concat();

    let mut command_objects = Commands {
        raw: String::from(args_str),
        cmds: Vec::new(),
    };

    if args.len() == 0 {
        //If no futher args exist print usage
        print_usage();
    }

    let mut iterator = args.iter();

    while let Some(arg) = iterator.next() {
        match arg.as_str() {
            "-h" => {
                let cmd = Cmd {
                    cmd_type: Cmds::Usage,
                    args: None,
                };
                command_objects.cmds.push(cmd);
                print_usage();
            }

            "-l" => {
                let cmd = Cmd {
                    cmd_type: Cmds::ListShortcuts,
                    args: None,
                };
                command_objects.cmds.push(cmd);
                process_list_shortcuts();
            }

            "-a" => {
                let cmd = Cmd {
                    cmd_type: Cmds::AddShortcut,
                    args: match iterator.next() {
                        Some(x) => Some(vec![x.to_string()]),
                        None => None,
                    },
                };
                if let Some(ref args) = cmd.args {
                    process_add_shortcut(args);
                } else {
                    eprintln!("Error: No shortcut provided for -a option");
                }
                command_objects.cmds.push(cmd);
            }
            _ => {
                // If the argument is a shortcut, execute goto
                if args.len() == 1 {
                    process_goto(arg, &load_shortcuts());
                    return None;
                }
                // If not then print usage. 
                print_usage();
                return None;
            }
        }
    }
    
    Some(command_objects) //Return Commands
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    //dbg!(args);
    let commands = process_args(&mut args);
    //dbg!(commands);

}
