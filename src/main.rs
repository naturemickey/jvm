extern crate java;

use java::cmd::Cmd;
use java::classpath::Classpath;
use java::util::file_util;
use java::interpreter::*;
use java::rtda::heap::ClassLoader;

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
    let classpath = Classpath::parse(&cmd.cp_option);
    println!(" classpath:{} class:{} args:{:?}\n", cmd.cp_option.to_string(), cmd.class.to_string(), cmd.args);

    let classloader = ClassLoader::new(classpath);

    let class_name = file_util::convert_classname(&cmd.class);

    //println!("{}", class_name);

    let main_class = ClassLoader::load_class(classloader.clone(), &dbg!(class_name));
    let main_class_ref = main_class.read().unwrap();

    let main_method = main_class_ref.get_main_method();

    match main_method {
        Some(m) => interpret(m.as_ref()),
        None => println!("Main method not found in class {}", &cmd.class),
    }
//    let class_data = classpath.read_class(class_name);
//    match class_data {
//        Some((data, _)) => println!("class data:{:?}", data),
//        None => println!("could not find or load main class {}", cmd.class),
//    }
//    let class_file = load_class(&class_name, &classpath);
//
//    print_class_info(&class_file);
//
//    let om = get_main_method(&class_file);
//    match om {
//        Some(m) => interpret(m),
//        None => println!("Main method not found in class {}", &cmd.class),
//    }
}

//fn load_class(class_name: &str, classpath: &Classpath) -> ClassFile {
//    let (class_data, _) = classpath.read_class(class_name).unwrap();
//    ClassFile::parse(class_data)
//}
//
//fn get_main_method(class_file: &ClassFile) -> Option<&MemberInfo> {
//    for m in class_file.methods() {
//        if m.name() == "main" && m.descriptor() == "([Ljava/lang/String;)V" {
//            return Some(dbg!(m));
//        }
//    }
//    None
//}
//
//fn print_class_info(class_file: &ClassFile) {
//    println!("version: {}.{}", class_file.major_version(), class_file.minor_version());
//    println!("constants count: {}", class_file.constant_pool().constants_count());
//    println!("access flags: 0x{:x}", class_file.access_flags());
//    println!("this class: {}", class_file.class_name());
//    println!("super class: {}", class_file.super_class_name());
//    println!("intrefaces: {:?}", class_file.interface_names());
//    let fields = class_file.fields();
//    println!("fields count: {}", fields.len());
//    for field in fields {
//        println!("    {}", field.name());
//    }
//    let methods = class_file.methods();
//    println!("methods count: {}", methods.len());
//    for method in methods {
//        println!("    {}{}", method.name(), method.descriptor());
//    }
//}