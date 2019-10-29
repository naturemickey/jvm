pub struct Classpath {
    // boot_classpath,
    // ext_classpath,
    user_classpath: Box<dyn Entry>
}

impl Classpath {
    pub fn parse(cp_option: String) -> Classpath {
        let entry: Box<dyn Entry> = new_entry(cp_option);
        Self { user_classpath: entry }
    }
    pub fn read_class(&self, class_name: String) -> Option<(Vec<u8>, &dyn Entry)> {
        self.user_classpath.read_class(class_name)
    }
}

impl ToString for Classpath {
    fn to_string(&self) -> String {
        self.user_classpath.to_string()
    }
}