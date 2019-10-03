struct WildcardEntry {
    entry: CompositeEntry
}

impl WildcardEntry {
    fn new(path: String) -> WildcardEntry {
        let mut p = path.to_string();
        p.remove(path.len() - 1);
        let read_dir = std::fs::read_dir(p).unwrap();

        let mut paths: Vec<String> = Vec::new();

        for result_dir_entry in read_dir {
            let path_in = result_dir_entry.unwrap().path();

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

impl Entry for WildcardEntry {
    fn read_class(&self, class_name: String) -> Option<(Vec<u8>, &dyn Entry)> {
        self.entry.read_class(class_name)
    }
}

impl ToString for WildcardEntry {
    fn to_string(&self) -> String {
        self.entry.to_string()
    }
}