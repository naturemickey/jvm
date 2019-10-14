struct Class<'a> {
    access_flags: u16,
    name: &'a str,
    super_class_name: &'a str,
    interface_names: Vec<&'a str>,
    constant_pool: Arc<ConstantPool>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    //    loader: &'a ClassLoader,
//    super_class: &'a Class<'a>,
//    interfaces: Vec<&'a Class<'a>>,
//    instance_slot_count: usize,
//    static_slot_count: usize,
    static_vars: Slots,
}

impl<'a> Class<'a> {
    pub fn new(cf: &'a ClassFile) -> Box<Class<'a>> {
        let access_flags = cf.access_flags();
        let name = cf.class_name();
        let super_class_name = cf.super_class_name();
        let interface_names = cf.interface_names();
        let fields = Field::new_fields();
        let methods = Method::new_methods();
        // todo loader
        // todo super_class
        // todo interfaces
        // todo instance_slot_count
        // todo static_slot_count
        let static_vars = Slots::new();
        Box::new(Self { access_flags, name, super_class_name, interface_names, constant_pool: cf.constant_pool(), fields, methods, static_vars })
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }
    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }
    pub fn is_super(&self) -> bool {
        self.access_flags & ACC_SUPER != 0
    }
    pub fn is_interface(&self) -> bool {
        self.access_flags & ACC_INTERFACE != 0
    }
    pub fn is_abstract(&self) -> bool {
        self.access_flags & ACC_ABSTRACT != 0
    }
    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }
    pub fn is_annotation(&self) -> bool {
        self.access_flags & ACC_ANNOTATION != 0
    }
    pub fn is_enum(&self) -> bool {
        self.access_flags & ACC_ENUM != 0
    }

    pub fn constant_pool(&self) -> Arc<ConstantPool> {
        self.constant_pool.clone()
    }
    fn static_vars(&self) -> &Slots {
        &self.static_vars
    }

    pub fn is_accessible_to(&self, other: &Class) -> bool {
        self.is_public() || self.package_name() == other.package_name()
    }
    pub fn package_name(&self) -> &str {
        match self.name.rfind("/") {
            Some(i) => {
                self.name.get(..i).unwrap()
            }
            None => "",
        }
    }

    pub fn get_main_method(&self) -> Option<&Method> {
        self.get_static_method("main", "([Ljava/lang/String;)V")
    }

    pub fn get_static_method(&self, name: &str, descriptor: &str) -> Option<&Method> {
        for method in &self.methods {
            if method.is_static() && method.name() == name && method.descriptor() == descriptor {
                return Some(method);
            }
        }
        None
    }

    pub fn new_object(&self) -> Arc<Object> {
        Object::new(self)
    }
}
