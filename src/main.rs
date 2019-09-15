extern crate java;

use java::cmd::Cmd;
use java::classpath::Classpath;
use java::util::file_util;

fn main() {
    let cmd = Cmd::parse();

    if cmd.version_fle {
        println!("version 0.0.1");
    } else if cmd.help_flg {
        println!("Usage: {} [-options] class [args...]\n", cmd.class);
    } else {
        start_jvm(cmd);
    }
}

fn start_jvm(cmd: Cmd) {
    let cp = Classpath::parse(cmd.cp_option.to_string());
    println!(" classpath:{} class:{} args:{:?}\n", cmd.cp_option.to_string(), cmd.class.to_string(), cmd.args);

    let class_name = file_util::classname_to_filename(&cmd.class);

    // println!("{}", class_name);
    let class_data = cp.read_class(class_name);
    match class_data {
        Some((data, _)) => println!("class data:{:?}", data),
        None => println!("could not find or load main class {}", cmd.class),
    }
}
