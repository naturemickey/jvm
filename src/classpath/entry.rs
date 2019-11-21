//pub trait Entry: ToString {
//    fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, Arc<dyn Entry>)>;
//}

pub enum Entry {
    Composite(CompositeEntry),
    Wildcard(WildcardEntry),
    Zip(ZipEntry),
    Dir(DirEntry),
}

impl Entry {
    pub fn new(path: &str) -> Entry {
        if dbg!(path).contains(SEPARATOR) {
            Entry::Composite(CompositeEntry::new(path))
        } else if path.ends_with("*") {
            Entry::Wildcard(WildcardEntry::new(path))
        } else if file_util::is_jar_name(path) {
            Entry::Zip(ZipEntry::new(path))
        } else {
            Entry::Dir(DirEntry::new(path))
        }
    }

    pub fn read_class(&self, class_name: &str) -> Option<Vec<u8>> {
        static CLASSNAME_SUFFIX: &str = ".class";
        let class_name = &if class_name.ends_with(CLASSNAME_SUFFIX) {
            class_name.to_string()
        } else {
            class_name.to_string() + CLASSNAME_SUFFIX
        };
        match dbg!(self) {
            Entry::Composite(e) => e.read_class(class_name),
            Entry::Wildcard(e) => e.read_class(class_name),
            Entry::Zip(e) => e.read_class(class_name),
            Entry::Dir(e) => e.read_class(class_name),
        }
    }
}

impl ToString for Entry {
    fn to_string(&self) -> String {
        match self {
            Entry::Composite(e) => "Entry::Composite[".to_owned() + &e.to_string() + "]",
            Entry::Wildcard(e) => "Entry::Wildcard[".to_owned() + &e.to_string() + "]",
            Entry::Zip(e) => "Entry::Zip[".to_owned() + &e.to_string() + "]",
            Entry::Dir(e) => "Entry::Dir[".to_owned() + &e.to_string() + "]",
        }
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}