
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::BufReader;
use std::error::Error;

pub fn path_to_file(path:&Path) -> File {
    File::open(path).unwrap()
}

pub fn read_file(file:&File) -> Vec<u8> {
    let mut reader = BufReader::new(file);
    let mut v = Vec::new();
    let result = reader.read_to_end(&mut v);
    if result.is_err() {
        panic!("xxxxxx {}", result.unwrap_err().description());
    }
    v
}

pub fn classname_to_filename(classname:&str) -> &str {
    classname
}