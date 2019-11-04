pub struct ClassLoader {
    classpath: Classpath,
    class_map: HashMap<String, Arc<Class>>,
}

impl ClassLoader {
    pub fn new(classpath: Classpath) -> Arc<ClassLoader> {
        Arc::new(Self { classpath, class_map: HashMap::new() })
    }

    pub fn load_class(loader:Arc<ClassLoader>, name: &str) -> Arc<Class> {
        match loader.class_map.get(name) {
            Some(class) => class.clone(),
            None => {
                // todo array_class?
                Self::load_non_array_class(loader, name)
            }
        }
    }

    fn load_non_array_class(loader:Arc<ClassLoader>, name: &str) -> Arc<Class> {
        let (data, entry) = loader.read_class(name);
        let class = loader.define_class(data);
        Self::link(class.clone());
        println!("[Loaded {} from {}", name, unsafe { &*entry }.to_string());
        return class.clone();
    }

    fn read_class(&self, name: &str) -> (Vec<u8>, Arc<dyn Entry>) {
        match self.classpath.read_class(name) {
            Some(res) => res.clone(),
            None => panic!("java.lang.ClassNotFoundException: {}", name)
        }
    }

    fn define_class(loader: Arc<ClassLoader>, data: Vec<u8>) -> Arc<Class> {
        let mut class = Self::parse_class(data, loader);
        Self::resolve_super_class(&mut class);
        Self::resolve_interfaces(&mut class);

        let class_name = class.name.to_string();

        loader.class_map.insert(class_name.clone(), class.clone());
        class
    }

    fn parse_class(data: Vec<u8>, loader: Arc<ClassLoader>) -> Arc<Class> {
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

    fn link(class: Arc<Class>) {
        let class = unsafe { &mut *(Arc::into_raw(class) as *mut Class) };
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
        let mut slot_id = match &class.super_class {
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
                "Z" | "B" | "C" | "S" | "I" => {
                    match cp.get_constant(cp_index) {
                        Constant::Integer(val) => vars.set_int(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                "J" => {
                    match cp.get_constant(cp_index) {
                        Constant::Long(val) => vars.set_long(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                "F" => {
                    match cp.get_constant(cp_index) {
                        Constant::Float(val) => vars.set_float(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                "D" => {
                    match cp.get_constant(cp_index) {
                        Constant::Double(val) => vars.set_double(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                "Ljava/lang/String" => {
                    panic!("todo")
                }
                _ => {
                    panic!("impossible.")
                }
            }
        }
        unimplemented!()
    }
}
