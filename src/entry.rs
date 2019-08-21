use super::os;
use super::util::file_util;
use std::path::Path;
use std::fs::File;

pub trait entry {
    fn to_string(&self) -> &str;
    fn read_class(&self, className: &str) -> (&Vec<u8>, &entry);
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
    fn new(path: &str) -> &zip_entry {}
}

impl composite_entry {
    fn new(path: &str) -> composite_entry {}
}

impl wildcard_entry {
    fn new(path: &str) -> wildcard_entry {}
}

impl entry for dir_entry {
    fn to_string(&self) -> &str {
        self.abs_dir.to_string_lossy().as_ref()
    }
    fn read_class<'a>(&self, className: &str) -> (&Vec<u8>, &'a entry) {
        let path = self.abs_dir.join(className).as_path();
        let file = file_util::path_to_file(path);
        (file_util::read_file(file), self)
    }
}

impl entry for zip_entry {}

impl entry for composite_entry {}

impl entry for wildcard_entry {}