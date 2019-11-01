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
        Self::link(unsafe { &mut *(class as *mut Class) });
        println!("[Loaded {} from {}", name, unsafe { &*entry }.to_string());
        return unsafe { &*class };
    }

    fn read_class(&self, name: &str) -> (Vec<u8>, *const dyn Entry) {
        match self.classpath.read_class(name) {
            Some(res) => res,
            None => panic!("java.lang.ClassNotFoundException: {}", name)
        }
    }

    fn define_class(&mut self, data: Vec<u8>) -> *const Class {
        let mut class = Self::parse_class(data, self);
        Self::resolve_super_class(&mut class);
        Self::resolve_interfaces(&mut class);

        let class_name = class.name.to_string();
        self.class_map.insert(class_name.clone(), class);
        self.class_map.get(&class_name).unwrap()
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

            for i in 0..interface_count {
                let interface_name = &class.interface_names[i].clone();
                let interface_ptr = class.loader_mut().load_class(interface_name);
                class.interfaces.push(interface_ptr);
            }
        }
    }

    fn link(class: &mut Class) {
        Self::verify(class);
        Self::prepare(class);
    }

    fn verify(class: &Class) {
        // todo
    }

    fn prepare(class: &mut Class) {
        Self::calc_instance_field_slot_ids(class);
        Self::calc_static_field_slot_ids(class);
        Self::alloc_and_init_static_vars(class);
    }

    fn calc_instance_field_slot_ids(class: &mut Class) {
        let mut slot_id = match class.super_class {
            None => 0,
            Some(c) => unsafe { &*c }.instance_slot_count
        };
        for field in &mut class.fields {
            if !field.is_static() {
                field.slot_id = slot_id;
                slot_id += if field.is_long_or_double() { 2 } else { 1 };
            }
        }
        class.instance_slot_count = slot_id;
    }

    fn calc_static_field_slot_ids(class: &mut Class) {
        let mut slot_id = 0usize;
        for field in &mut class.fields {
            field.slot_id = slot_id;
            slot_id += if field.is_long_or_double() { 2 } else { 1 };
        }
        class.static_slot_count = slot_id;
    }

    fn alloc_and_init_static_vars(class: &mut Class) {
        let slots = Slots::new(class.static_slot_count);
        let class_ptr: *mut Class = class;
        for field in &class.fields {
            if field.is_static() && field.is_final() {
                Self::init_static_final_var(class_ptr, field);
            }
        }
        class.static_vars = slots
    }

//    fn init_static_final_var(class: &mut Class, cp_index: u16, slot_id: usize, descriptor: String) {
//        unimplemented!()
//    }

    fn init_static_final_var(class: *mut Class, field: &Field) {
        let vars = &mut unsafe { &mut *class }.static_vars;
        let cp = unsafe { &*class }.constant_pool();
        let cp_index = field.const_value_index();
        let slot_id = field.slot_id();
        let descriptor = field.descriptor();
        if cp_index > 0 {
            match descriptor {
                "Z" | "B" | "C" | "S" | "I" => {}
                "J" => {}
                "F" => {}
                "D" => {}
                "Ljava/lang/String" => {}
                _ => {}
            }
        }
        unimplemented!()
    }
}
