use super::os;
use super::util::file_util;
use std::path::Path;
use std::fs::File;
use std::option::Option;
use crate::os::path_list_separator;

pub trait entry {
    fn to_string(&self) -> &str;
    fn read_class(&self, className: &str) -> Option<(&Vec<u8>, &entry)>;
}

struct dir_entry<'a> {
    abs_dir: &'a Path
}

struct zip_entry<'a> {
    abs_path: &'a Path
}

struct composite_entry<'a> {
    entrys: Vec<&'a entry>
}

struct wildcard_entry<'a> {
    entry: composite_entry<'a>
}

pub fn new_entry(path: &str) -> &entry {
    if path.contains(os::path_list_separator) {
        composite_entry::new(path)
    } else if path.ends_with("*") {
        wildcard_entry::new(path)
    } else if path.ends_with(".jar") || path.ends_with(".JAR") || path.ends_with(".zip") || path.ends_with(".ZIP") {
        zip_entry::new(path)
    } else {
        dir_entry::new(path)
    }
}

impl dir_entry {
    fn new(path: &str) -> &dir_entry {
        &Self { abs_dir: Path::new(path) }
    }
}

impl zip_entry {
    fn new(path: &str) -> &zip_entry {
        &Self { abs_path: Path::new(path) }
    }
}

impl composite_entry {
    fn new(path: &str) -> composite_entry {
        let path_list = path.split(path_list_separator);
        let mut path_vec = Vec::new();
        for p in path_list {

        }
    }
}

impl wildcard_entry {
    fn new(path: &str) -> wildcard_entry {}
}

impl entry for dir_entry {
    fn to_string(&self) -> &str {
        self.abs_dir.to_string_lossy().as_ref()
    }
    fn read_class<'a>(&self, className: &str) -> Option<(&Vec<u8>, &entry)> {
        let path = self.abs_dir.join(className).as_path();
        if path.is_file() {
            let file = file_util::path_to_file(path);
            Some((file_util::read_file(file), self))
        } else {
            None
        }
    }
}

impl entry for zip_entry {
    fn to_string(&self) -> &str {
        self.abs_path.to_string_lossy().as_ref()
    }
    fn read_class<'a>(&self, className: &str) -> (&Vec<u8>, &'a entry) {

    }
}

impl entry for composite_entry {
    fn to_string(&self) -> &str {

    }
    fn read_class<'a>(&self, className: &str) -> (&Vec<u8>, &'a entry) {

    }
}

impl entry for wildcard_entry {
    fn to_string(&self) -> &str {

    }
    fn read_class<'a>(&self, className: &str) -> (&Vec<u8>, &'a entry) {

    }
}