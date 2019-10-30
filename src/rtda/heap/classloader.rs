pub struct ClassLoader {
    classpath: Classpath,
    classmap: HashMap<String, Class>,
}

impl ClassLoader {
    pub fn new(classpath: Classpath) -> ClassLoader {
        Self { classpath, classmap: HashMap::new() }
    }

    pub fn load_class(&mut self, name: &str) -> *const Class {
        match self.classmap.get(name) {
            Some(class) => class,
            None => {
                // todo array_class?
                self.load_non_array_class(name)
            }
        }
    }

    fn load_non_array_class(&mut self, name: &str) -> &Class {
        let (data, entry) = self.read_class(name);
        let class = self.define_class(data);
        Self::link(class);
        println!("[Loaded {} from {}", name, entry.to_string());
        return class;
    }

    fn read_class(&self, name: &str) -> (Vec<u8>, &dyn Entry) {
        match self.classpath.read_class(name) {
            Some(res) => res,
            None => panic!("java.lang.ClassNotFoundException: {}" , name)
        }
    }

    fn define_class(&mut self, data: Vec<u8>) -> &Class {
        unimplemented!()
    }

    fn parse_class(data: Vec<u8>) -> Class {
        unimplemented!()
    }

    fn resolve_super_class(class: &Class) {
        unimplemented!()
    }

    fn resolve_interfaces(class: &Class) {
        unimplemented!()
    }

    fn link(class: &Class) {
        Self::verify(class);
        Self::prepare(class);
    }

    fn verify(class: &Class) {
        unimplemented!()
    }

    fn prepare(class: &Class) {
        unimplemented!()
    }

    fn calc_instance_field_slot_ids(class: &Class) {
        unimplemented!()
    }

    fn calc_static_field_slot_ids(class: &Class) {
        unimplemented!()
    }

    fn alloc_and_init_static_vars(class: &Class) {
        unimplemented!()
    }

    fn init_static_final_var(class: &Class, field: &Field) {
        unimplemented!()
    }
}