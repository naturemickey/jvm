extern crate java;

use java::cmd;
use java::cmd::Cmd;
use java::entry;

fn main() {
    let cmd = cmd::parse_cmd();

    if cmd.version_fle {
        println!("version 0.0.1");
    } else if cmd.help_flg {
        println!("Usage: {} [-options] class [args...]\n", cmd.class);
    } else {
        start_jvm(cmd);
    }
}

fn start_jvm(cmd: Cmd) {
    entry::new_entry(cmd.cp_option);
    println!(" classpath:{} class:{} args:{:?}\n", cmd.cp_option, cmd.class, cmd.args);
}
