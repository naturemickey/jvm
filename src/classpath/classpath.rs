pub struct Classpath {
    // boot_classpath,
    // ext_classpath,
    user_classpath: Arc<dyn Entry>
}

impl Classpath {
    pub fn parse(cp_option: &str) -> Classpath {
        Self { user_classpath: new_entry(cp_option) }
    }
    pub fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, Arc<dyn Entry>)> {
        self.user_classpath.read_class(class_name)
    }
}

impl ToString for Classpath {
    fn to_string(&self) -> String {
        self.user_classpath.to_string()
    }
}