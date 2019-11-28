pub struct ClassLoader {
    classpath: Classpath,
    class_map: HashMap<String, Arc<RwLock<Class>>>,
}

impl ClassLoader {
    pub fn new(classpath: Classpath) -> Arc<RwLock<ClassLoader>> {
        Arc::new(RwLock::new(Self { classpath, class_map: HashMap::new() }))
    }

    pub fn load_class(loader: Arc<RwLock<ClassLoader>>, name: &str) -> Arc<RwLock<Class>> {
        match loader.read().unwrap().class_map.get(name) {
            Some(class) => class.clone(),
            None => {
                // todo array_class?
                Self::load_non_array_class(loader.clone(), name)
            }
        }
    }

    fn load_non_array_class(loader: Arc<RwLock<ClassLoader>>, name: &str) -> Arc<RwLock<Class>> {
        let data = loader.read().unwrap().read_class(name);
        let class = Self::define_class(loader, data);
        Self::link(class.clone());

        return class.clone();
    }

    fn read_class(&self, name: &str) -> Vec<u8> {
        match self.classpath.read_class(name) {
            Some(res) => res,
            None => panic!("java.lang.ClassNotFoundException: {}", name)
        }
    }

    fn define_class(loader: Arc<RwLock<ClassLoader>>, data: Vec<u8>) -> Arc<RwLock<Class>> {
        let class = Self::parse_class(data, loader.clone());
        let class_ref = &mut class.write().unwrap();
        let loader_ref = &mut loader.write().unwrap();

        Self::resolve_super_class(class_ref);
        Self::resolve_interfaces(class_ref);

        let class_name = class_ref.name.to_string();

        loader_ref.class_map.insert(class_name.clone(), class.clone());
        class.clone()
    }

    fn parse_class(data: Vec<u8>, loader: Arc<RwLock<ClassLoader>>) -> Arc<RwLock<Class>> {
        let cf = classfile::ClassFile::parse(data);
        Class::new(&cf, loader)
    }

    fn resolve_super_class(class: &mut Class) {
        if class.name.ne(&OBJECT_CLASS_NAME.to_string()) {
            let super_class_name = class.super_class_name.clone();
            class.super_class = Some(Self::load_class(class.loader(), &super_class_name));
        }
    }

    fn resolve_interfaces(class: &mut Class) {
        let interface_count = class.interface_names.len();
        if interface_count > 0 {
            class.interfaces = Vec::with_capacity(interface_count);

            for i in 0..interface_count {
                let interface_name = &class.interface_names[i].clone();
                let interface_ptr = Self::load_class(class.loader(), interface_name);
                class.interfaces.push(interface_ptr);
            }
        }
    }

    fn link(class: Arc<RwLock<Class>>) {
        Self::verify(class.clone());
        Self::prepare(class);
    }

    fn verify(class: Arc<RwLock<Class>>) {
        // todo
    }

    fn prepare(class: Arc<RwLock<Class>>) {
        let class = &mut class.write().unwrap();
        Self::calc_instance_field_slot_ids(class);
        Self::calc_static_field_slot_ids(class);
        Self::alloc_and_init_static_vars(class);
    }

    fn calc_instance_field_slot_ids(class: &mut Class) {
        let mut slot_id = match &class.super_class {
            None => 0,
            Some(c) => c.read().unwrap().instance_slot_count
        };
        for field in &mut class.fields {
            if !field.read().unwrap().is_static() {
                let mut field_ref = field.write().unwrap();
                field_ref.slot_id = slot_id;
                slot_id += if field.read().unwrap().is_long_or_double() { 2 } else { 1 };
            }
        }
        class.instance_slot_count = slot_id;
    }

    fn calc_static_field_slot_ids(class: &mut Class) {
        let mut slot_id = 0usize;
        for field in &mut class.fields {
            let mut field_ref = field.write().unwrap();
            field_ref.slot_id = slot_id;
            slot_id += if field.read().unwrap().is_long_or_double() { 2 } else { 1 };
        }
        class.static_slot_count = slot_id;
    }

    fn alloc_and_init_static_vars(class: &mut Class) {
        let slots = Slots::new(class.static_slot_count);
        let class_ptr: *mut Class = class;
        for field in &class.fields {
            if field.read().unwrap().is_static() && field.read().unwrap().is_final() {
                Self::init_static_final_var(class_ptr, &field.read().unwrap());
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
                    match cp.read().unwrap().get_constant(cp_index) {
                        Constant::Integer(val) => vars.set_int(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                "J" => {
                    match cp.read().unwrap().get_constant(cp_index) {
                        Constant::Long(val) => vars.set_long(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                "F" => {
                    match cp.read().unwrap().get_constant(cp_index) {
                        Constant::Float(val) => vars.set_float(slot_id, *val),
                        _ => panic!("impossible.")
                    }
                }
                "D" => {
                    match cp.read().unwrap().get_constant(cp_index) {
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
