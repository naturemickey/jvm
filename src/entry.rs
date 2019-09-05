use crate::os;
use crate::util::file_util;
use std::path::Path;
use std::fs::File;
use std::option::Option;
use crate::os::*;
use std::string::ToString;
use std::io::{BufReader, Read};
use std::error::Error;

pub trait Entry {
    fn read_class(&self, className: &str) -> Option<(&Vec<u8>, &Entry)>;
}

struct DirEntry<'a> {
    abs_dir: &'a Path
}

struct ZipEntry<'a> {
    abs_path: &'a Path
}

struct CompositeEntry<'a> {
    entrys: Vec<&'a Entry>
}

struct WildcardEntry<'a> {
    entry: CompositeEntry<'a>
}

pub fn new_entry(path: &str) -> &Entry {
    if path.contains(os::path_list_separator) {
        &CompositeEntry::new(path)
    } else if path.ends_with("*") {
        &WildcardEntry::new(path)
    } else if path.ends_with(".jar") || path.ends_with(".JAR") || path.ends_with(".zip") || path.ends_with(".ZIP") {
        &ZipEntry::new(path)
    } else {
        &DirEntry::new(path)
    }
}

impl<'a> DirEntry<'a> {
    fn new(path: &'a str) -> DirEntry<'a> {
        Self { abs_dir: &Path::new(path) }
    }
}

impl<'a> ZipEntry<'a> {
    fn new(path: &'a str) -> ZipEntry<'a> {
        Self { abs_path: &Path::new(path) }
    }
}

impl<'a> CompositeEntry<'a> {
    fn new(path: &'a str) -> CompositeEntry<'a> {
        let mut path_vec: Vec<&Entry> = Vec::new();
        let pl: Vec<&'a str> = path.split(path_list_separator).collect();
        for p in pl {
            path_vec.push(new_entry(p));
        }
        Self { entrys: path_vec }
    }
}

impl<'a> WildcardEntry<'a> {
    fn new(path: &'a str) -> WildcardEntry<'a> {
        Self { entry: CompositeEntry::new(path) }
    }
}

impl<'a> Entry for DirEntry<'a> {
    fn read_class(&self, className: &str) -> Option<(&Vec<u8>, &Entry)> {
        let path = self.abs_dir.join(className).as_path();
        if path.is_file() {
            let file = file_util::path_to_file(path);
            Some((file_util::read_file(file), self))
        } else {
            None
        }
    }
}

impl<'a> Entry for ZipEntry<'a> {
    fn read_class(&self, className: &str) -> Option<(&Vec<u8>, &Entry)> {
        let file = file_util::path_to_file(self.abs_path);
        let classFileName = file_util::classname_to_filename(className);
        let reader = BufReader::new(file);
        let mut za = zip::ZipArchive::new(reader).unwrap();

        let mut file = match za.by_name(classFileName)
            {
                Ok(file) => file,
                Err(why) => panic!("{}", why)
            };
        let mut v = Vec::new();
        file.read_to_end(&mut v);
        Some((&v, self))
    }
}

impl<'a> Entry for CompositeEntry<'a> {
    fn read_class(&self, className: &str) -> Option<(&Vec<u8>, &Entry)> {}
}

impl<'a> Entry for WildcardEntry<'a> {
    fn read_class(&self, className: &str) -> Option<(&Vec<u8>, &Entry)> {}
}

impl<'a> ToString for DirEntry<'a> {
    fn to_string(&self) -> String {
        self.abs_dir.to_string_lossy().as_ref().to_string()
    }
}

impl<'a> ToString for ZipEntry<'a> {
    fn to_string(&self) -> String {
        self.abs_path.to_string_lossy().as_ref().to_string()
    }
}

impl<'a> ToString for CompositeEntry<'a> {
    fn to_string(&self) -> String {
        let mut strs = Vec::new();
        for entry in self.entrys {
            strs.push(entry.to_string());
        }
        strs.join(path_list_separator_str)
    }
}

impl<'a> ToString for WildcardEntry<'a> {
    fn to_string(&self) -> String {
        self.entry.to_string()
    }
}