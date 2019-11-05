pub struct ZipEntry {
    abs_path: String
}

impl ZipEntry {
    fn new(path: &str) -> ZipEntry {
        Self { abs_path: path.to_string() }
    }
    fn read_class(&self, class_name: &str) -> Option<Vec<u8>> {
        let file = File::open(Path::new(&self.abs_path)).unwrap();
        let reader = BufReader::new(file);
        let mut za = zip::ZipArchive::new(reader).unwrap();

        // println!("aaaaaaaaaaaaaaaaaaaaaaaaa {}", class_name);

        let zf = za.by_name(class_name);
        match zf {
            Err(_) => None,
            Ok(mut file) => {
                let mut v = Vec::new();
                let read_res = file.read_to_end(&mut v);
                if read_res.is_err() {
                    panic!("ZipEntry read file err {}", read_res.unwrap_err().description());
                }
                Some(v)
            }
        }
    }
}


impl ToString for ZipEntry {
    fn to_string(&self) -> String {
        self.abs_path.to_string()
    }
}