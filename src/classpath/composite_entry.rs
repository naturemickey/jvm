pub struct CompositeEntry {
    entrys: Vec<Entry>
}

impl CompositeEntry {
    fn new(path: &str) -> CompositeEntry {
        let paths: Vec<&str> = path.split(SEPARATOR).collect();
        let mut paths2 = Vec::new();
        for p in paths {
            paths2.push(p.to_string());
        }
        Self::new_by_paths(paths2)
    }
    fn new_by_paths(paths: Vec<String>) -> CompositeEntry {
        let mut entrys = Vec::new();
        for p in paths {
            // println!("{}", p);
            entrys.push(Entry::new(&p));
        }
        Self::new_by_entrys(entrys)
    }
    fn new_by_entrys(entrys: Vec<Entry>) -> CompositeEntry {
        Self { entrys }
    }
    fn read_class(&self, class_name: &str) -> Option<Vec<u8>> {
        for entry in &self.entrys {
            let res = entry.read_class(class_name);
            if res.is_some() {
                return res;
            }
        }
        return None;
    }
}


impl ToString for CompositeEntry {
    fn to_string(&self) -> String {
        let mut strs = Vec::new();
        for entry in &self.entrys {
            strs.push(entry.to_string());
        }
        strs.join(SEPARATOR_STR)
    }
}