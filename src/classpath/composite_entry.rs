struct CompositeEntry {
    entrys: Vec<Arc<dyn Entry>>
}

impl CompositeEntry {
    fn new(path: &str) -> CompositeEntry {
        let paths: Vec<&str> = path.split(if cfg!(windows) { ';' } else { ':' }).collect();
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
            entrys.push(new_entry(&p));
        }
        Self::new_by_entrys(entrys)
    }
    fn new_by_entrys(entrys: Vec<Arc<dyn Entry>>) -> CompositeEntry {
        Self { entrys }
    }
}

impl Entry for CompositeEntry {
    fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, Arc<dyn Entry>)> {
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
        strs.join(if cfg!(windows) { ";" } else { ":" })
    }
}