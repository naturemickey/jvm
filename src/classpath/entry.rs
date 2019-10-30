pub trait Entry: ToString {
    fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, &dyn Entry)>;
}

pub fn new_entry(path: &str) -> Box<dyn Entry> {
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