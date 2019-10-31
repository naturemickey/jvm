pub struct ClassLoader {
    classpath: Classpath,
    class_map: HashMap<String, Class>,
}

impl ClassLoader {
    pub fn new(classpath: Classpath) -> ClassLoader {
        Self { classpath, class_map: HashMap::new() }
    }

    pub fn load_class(&mut self, name: &str) -> *const Class {
        match self.class_map.get(name) {
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
        println!("[Loaded {} from {}", name, unsafe { &*entry }.to_string());
        return class;
    }

    fn read_class(&self, name: &str) -> (Vec<u8>, *const dyn Entry) {
        match self.classpath.read_class(name) {
            Some(res) => res,
            None => panic!("java.lang.ClassNotFoundException: {}", name)
        }
    }

    fn define_class(&mut self, data: Vec<u8>) -> &Class {
        let mut class = Self::parse_class(data, self);
        Self::resolve_super_class(&mut class);
        Self::resolve_interfaces(&mut class);

        let class_name = class.name.to_string();
        self.class_map.insert(class_name.clone(), class);
        &self.class_map.get(&class_name).unwrap()
    }

    fn parse_class(data: Vec<u8>, loader: &Self) -> Class {
        let cf = classfile::ClassFile::parse(data);
        Class::new(&cf, loader)
    }

    fn resolve_super_class(class: &mut Class) {
        if class.name.ne(&OBJECT_CLASS_NAME.to_string()) {
            let super_class_name = class.super_class_name.clone();
            class.super_class = Some(class.loader_mut().load_class(&super_class_name));
        }
    }

    fn resolve_interfaces(class: &mut Class) {
        let interface_count = class.interface_names.len();
        if interface_count > 0 {
            class.interfaces = Vec::with_capacity(interface_count);

            for i in 0 .. interface_count {
                let interface_name = &class.interface_names[i].clone();
                let interface_ptr = class.loader_mut().load_class(interface_name);
                class.interfaces.push(interface_ptr);
            }
        }
    }

    fn link(class: &Class) {
        Self::verify(class);
        Self::prepare(class);
    }

    fn verify(class: &Class) {
        // todo
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