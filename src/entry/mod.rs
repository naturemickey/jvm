use crate::util::file_util;
use std::path::Path;
use std::option::Option;
use std::string::ToString;
use std::io::{BufReader, Read};
use std::error::Error;
use std::fs::File;

pub trait Entry: ToString {
    fn read_class(&self, class_name: String) -> Option<(Vec<u8>, &dyn Entry)>;
}

pub fn new_entry(path: String) -> Box<dyn Entry> {
    if path.contains(if cfg!(windows) { ';' } else { ':' }) {
        Box::new(CompositeEntry::new(path))
    } else if path.ends_with("*") {
        Box::new(WildcardEntry::new(path))
    } else if file_util::is_jar_name(&path) {
        Box::new(ZipEntry::new(path))
    } else {
        Box::new(DirEntry::new(path))
    }
}

include!("composite_entry.rs");
include!("wildcard_entry.rs");
include!("zip_entry.rs");
include!("dir_entry.rs");