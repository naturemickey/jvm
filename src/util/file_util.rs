use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::error::Error;

pub fn read_file(file: &File) -> Vec<u8> {
    let mut reader = BufReader::new(file);
    let mut v = Vec::new();
    let result = reader.read_to_end(&mut v);
    if result.is_err() {
        panic!("xxxxxx {}", result.unwrap_err().description());
    }
    v
}

pub fn convert_classname(classname: &str) -> String {
    classname.replace(".", "/")
}

pub fn is_jar_name(file_name: &str) -> bool {
    file_name.ends_with(".jar") || file_name.ends_with(".JAR") || file_name.ends_with(".zip") || file_name.ends_with(".ZIP")
}