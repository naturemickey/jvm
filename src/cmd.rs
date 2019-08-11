pub struct Cmd {
    helpFlg: bool,
    versionFlg: bool,
    cpOption: String,
    class: String,
    args: Vec<String>,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd { helpFlg: false, versionFlg: false, cpOption: String::new(), class: String::new(), args: [] }
    }
}

use std::env;

pub fn parseCmd() -> Cmd {
    let mut cmd = Cmd::new();

    let mut setCp = false;

    for argument in env::args() {
        if cmd.class.is_empty() {
            cmd.class == argument;
            continue;
        }

        cmd.args.push(argument);

        if setCp {
            setCp = false;
            cmd.cpOption = argument;
            continue;
        }

        if &argument == "-help" || &argument == "-?" {
            println!("print help message.");
            cmd.helpFlg = true;
        }

        if &argument == "-version" {
            println!("print version and exit.");
            cmd.versionFlg = true;
        }

        if &argument == "-cp" || &argument == "-classpath" {
            setCp = true;
        }

        print!("Usage: %s [-options] class [args...]\n", cmd.class);
    }

    cmd
}