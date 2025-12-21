use std::env;

#[derive(Debug)]
enum Cmds {
    ListShortcuts,
    AddShortcut,
    DeleteShortcut,
    Goto,
    Usage,
}
#[derive(Debug)]
struct Cmd {
    cmd_type: Cmds,
    args: Option<Vec<String>>,
}

#[derive(Debug)]
struct Commands {
    raw: String,
    cmds: Vec<Cmd>,
}

fn print_usage() {
    println!("Usage: ");
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
                let mut cmd = Cmd {
                    cmd_type: Cmds::Usage,
                    args: None,
                };
                commandObjects.cmds.push(cmd);
            }

            "-l" => {
                let mut cmd = Cmd {
                    cmd_type: Cmds::ListShortcuts,
                    args: None,
                };
                commandObjects.cmds.push(cmd);
            }

            "-a" => {
                let mut cmd = Cmd {
                    cmd_type: Cmds::AddShortcut,
                    args: match iterator.next() {
                        Some(x) => Some(vec![x.to_string()]),
                        None => None,
                    },
                };
                commandObjects.cmds.push(cmd);
            }
            _ => {
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

    dbg!(commands);
}
