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

struct DirEntry {
    abs_dir: String
}

struct ZipEntry {
    abs_path: String
}

struct CompositeEntry<'a> {
    entrys: Vec<Box<dyn Entry + 'a>>
}

struct WildcardEntry<'a> {
    entry: CompositeEntry<'a>
}

pub fn new_entry<'a>(path: String) -> Box<dyn Entry + 'a> {
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

impl DirEntry {
    fn new(path: String) -> DirEntry {
        Self { abs_dir: path }
    }
}

impl ZipEntry {
    fn new(path: String) -> ZipEntry {
        Self { abs_path: path }
    }
}

impl<'a> CompositeEntry<'a> {
    fn new(path: String) -> CompositeEntry<'a> {
        let paths: Vec<&str> = path.split(if cfg!(windows) { ';' } else { ':' }).collect();
        let mut paths2: Vec<String> = Vec::new();
        for p in paths {
            paths2.push(p.to_string());
        }
        Self::new_by_paths(paths2)
    }
    fn new_by_paths(paths: Vec<String>) -> CompositeEntry<'a> {
        let mut entrys = Vec::new();
        for p in paths {
            // println!("{}", p);
            entrys.push(new_entry(p));
        }
        Self::new_by_entrys(entrys)
    }
    fn new_by_entrys(entrys: Vec<Box<dyn Entry + 'a>>) -> CompositeEntry<'a> {
        Self { entrys: entrys }
    }
}

impl<'a> WildcardEntry<'a> {
    fn new(path: String) -> WildcardEntry<'a> {
        let mut p = path.to_string();
        p.remove(path.len() - 1);
        let read_dir = std::fs::read_dir(p).unwrap();

        let mut paths: Vec<String> = Vec::new();

        for result_dir_entry in read_dir {
            let path_in  = result_dir_entry.unwrap().path();

            if path_in.is_file() {
                let file_name = path_in.file_name().unwrap().to_str().unwrap();

                if file_util::is_jar_name(file_name) {
                    paths.push(path_in.to_str().unwrap().to_string());
                }
            }
        }
        // println!("{:?}", paths);
        Self { entry: CompositeEntry::new_by_paths(paths) }
    }
}

impl Entry for DirEntry {
    fn read_class(&self, class_name: String) -> Option<(Vec<u8>, &dyn Entry)> {
        let pb = Path::new(&self.abs_dir).join(class_name);
        let path = pb.as_path();
        if path.is_file() {
            let file =  File::open(path).unwrap();
            Some((file_util::read_file(&file), self))
        } else {
            None
        }
    }
}

impl Entry for ZipEntry {
    fn read_class(&self, class_name: String) -> Option<(Vec<u8>, &dyn Entry)> {
        let file = File::open(Path::new(&self.abs_path)).unwrap();
        let reader = BufReader::new(file);
        let mut za = zip::ZipArchive::new(reader).unwrap();

        // println!("aaaaaaaaaaaaaaaaaaaaaaaaa {}", class_name);

        let zf = za.by_name(&class_name);
        if zf.is_err() {
            return None;
        }
        let mut file = zf.unwrap();
        let mut v = Vec::new();
        let read_res = file.read_to_end(&mut v);
        if read_res.is_err() {
            panic!("ZipEntry read file err {}", read_res.unwrap_err().description());
        }
        Some((v, self))
    }
}

impl<'a> Entry for CompositeEntry<'a> {
    fn read_class(&self, class_name: String) -> Option<(Vec<u8>, &dyn Entry)> {
        for entry in &self.entrys {
            let res = entry.read_class(class_name.clone());
            if res.is_some() {
                return res;
            }
        }
        return None;
    }
}

impl Entry for WildcardEntry<'_> {
    fn read_class(&self, class_name: String) -> Option<(Vec<u8>, &dyn Entry)> {
        self.entry.read_class(class_name)
    }
}

impl ToString for DirEntry {
    fn to_string(&self) -> String {
        self.abs_dir.to_string()
    }
}

impl ToString for ZipEntry {
    fn to_string(&self) -> String {
        self.abs_path.to_string()
    }
}

impl ToString for CompositeEntry<'_> {
    fn to_string(&self) -> String {
        let mut strs = Vec::new();
        for entry in &self.entrys {
            strs.push(entry.to_string());
        }
        strs.join(if cfg!(windows) { ";" } else { ":" })
    }
}

impl ToString for WildcardEntry<'_> {
    fn to_string(&self) -> String {
        self.entry.to_string()
    }
}