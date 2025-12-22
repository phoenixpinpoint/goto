use std::env;
use std::collections::HashMap;

pub mod commands;
use commands::*;


fn print_usage() {
    println!("Usage: ");
}

fn process_list_shortcuts() {
   let shortcuts = load_shortcuts();
    for (key, value) in shortcuts.iter() {
        println!("{} = {}", key, value);
    } 
}

fn process_add_shortcut(args: &Vec<String>) {
    // Placeholder for adding a shortcut
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

    let mut commandObjects = Commands {
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
                commandObjects.cmds.push(cmd);
            }

            "-l" => {
                let cmd = Cmd {
                    cmd_type: Cmds::ListShortcuts,
                    args: None,
                };
                commandObjects.cmds.push(cmd);
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
                commandObjects.cmds.push(cmd);
            }
            _ => {
                // If the argument is a shortcut, execute goto
                if(args.len() == 1) {
                    process_goto(arg, &load_shortcuts());
                    return None;
                }
                // If not then print usage. 
                print_usage();
                return None;
            }
        }
    }
    
    Some(commandObjects) //Return Commands
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    //dbg!(args);
    let commands = process_args(&mut args);
    //dbg!(commands);

}
