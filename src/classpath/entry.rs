pub trait Entry: ToString {
    fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, *const dyn Entry)>;
}

pub fn new_entry(path: &str) -> Arc<dyn Entry> {
    if path.contains(if cfg!(windows) { ';' } else { ':' }) {
        Arc::new(CompositeEntry::new(path))
    } else if path.ends_with("*") {
        Arc::new(WildcardEntry::new(path))
    } else if file_util::is_jar_name(&path) {
        Arc::new(ZipEntry::new(path))
    } else {
        Arc::new(DirEntry::new(path))
    }
}