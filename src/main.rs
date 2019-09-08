extern crate java;

use java::cmd::Cmd;
use java::classpath::Classpath;

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
    let cp = Classpath::parse(&cmd.cp_option);
    println!(" classpath:{} class:{} args:{:?}\n", cmd.cp_option, cmd.class, cmd.args);

    let class_name = &(cmd.class.replace("\\.", if cfg!(windows) { "\\" } else { "/" }) + ".class");
    let class_data = cp.read_class(class_name);
    match class_data {
        Some((data, _)) => println!("class data:{:?}", data),
        None => println!("could not find or load main class {}", cmd.class),
    }
}
