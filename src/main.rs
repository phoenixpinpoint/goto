use std::env;

enum Cmds {
    ListShortcuts,
    AddShortcut,
    DeleteShortcut,
    Goto,
    Usage
}

struct Cmd {
    cmd_type: Cmds,
    args: Vec<String>
}

struct Commands {
    raw: String,
    cmds: Vec<Cmd>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
