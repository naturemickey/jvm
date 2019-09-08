
use std::env;

pub struct Cmd {
    pub help_flg: bool,
    pub version_fle: bool,
    pub cp_option: String,
    pub class: String,
    pub args: Vec<String>,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd { help_flg: false, version_fle: false, cp_option: String::new(), class: String::new(), args: [].to_vec() }
    }
    pub fn parse() -> Cmd {
        let mut cmd = Cmd::new();

        let mut set_cp = false;
        let mut is_first = true;

        for argument in env::args() {
            if is_first {
                is_first = false;
                continue;
            }

            cmd.args.push(argument.clone());

            if set_cp {
                set_cp = false;
                cmd.cp_option = argument;
                continue;
            }

            if &argument == "-help" || &argument == "-?" {
                cmd.help_flg = true;
                continue;
            }

            if &argument == "-version" {
                cmd.version_fle = true;
                continue;
            }

            if &argument == "-cp" || &argument == "-classpath" {
                set_cp = true;
                continue;
            }

            if cmd.class.is_empty() && !argument.starts_with("-") {
                cmd.class = argument.clone();
                continue;
            } else {
                panic!("error arguments.");
            }

        }

        cmd
    }
}

