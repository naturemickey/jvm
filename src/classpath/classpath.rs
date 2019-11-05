pub struct Classpath {
    // boot_classpath,
    // ext_classpath,
    user_classpath: Entry
}

impl Classpath {
    pub fn parse(cp_option: &str) -> Classpath {
        Self { user_classpath: Entry::new(cp_option) }
    }
    pub fn read_class(&self, class_name: &str) -> Option<Vec<u8>> {
        self.user_classpath.read_class(class_name)
    }
}

impl ToString for Classpath {
    fn to_string(&self) -> String {
        self.user_classpath.to_string()
    }
}