pub struct Class<'a> {
    access_flags: u16,
    name: &'a str,
    super_class_name: &'a str,
    interface_names: Vec<&'a str>,
    constant_pool: Arc<ConstantPool<'a>>,
    fields: Vec<Field<'a>>,
    methods: Vec<Method<'a>>,
    //    loader: &'a ClassLoader,
//    super_class: &'a Class<'a>,
//    interfaces: Vec<&'a Class<'a>>,
//    instance_slot_count: usize,
//    static_slot_count: usize,
    static_vars: Slots,
}

impl<'a> Class<'a> {
    pub fn new(cf: &'a ClassFile) -> Arc<Class<'a>> {
        let access_flags = cf.access_flags();
        let name = cf.class_name();
        let super_class_name = cf.super_class_name();
        let interface_names = cf.interface_names();
        let mut constant_pool = Arc::new(ConstantPool::new(cf.constant_pool(), None));
        let fields = Vec::with_capacity(0);
        let methods = Vec::with_capacity(0);
        // todo loader
        // todo super_class
        // todo interfaces
        // todo instance_slot_count
        // todo static_slot_count
        let static_vars = Slots::new();
        let mut arc_class = Arc::new(Self { access_flags, name, super_class_name, interface_names, constant_pool: constant_pool.clone(), fields, methods, static_vars });

        Arc::get_mut(&mut constant_pool).unwrap().set_class(Some(arc_class.clone()));

        let methods = Method::new_methods(arc_class.clone(), cf.methods());
        let fields = Field::new_fields(arc_class.clone(), cf.fields());

        let class: &mut Self = Arc::get_mut(&mut arc_class).unwrap();
        class.methods = methods;
        class.fields = fields;

        arc_class
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

    pub fn constant_pool(&'a self) -> Arc<ConstantPool<'a>> {
        self.constant_pool.clone()
    }
    fn static_vars(&self) -> &Slots {
        &self.static_vars
    }

    pub fn is_accessible_to(&'a self, other: &'a Class<'a>) -> bool {
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

    pub fn get_main_method(&'a self) -> Option<&'a Method<'a>> {
        self.get_static_method("main", "([Ljava/lang/String;)V")
    }

    pub fn get_static_method(&'a self, name: &'a str, descriptor: &'a str) -> Option<&'a Method<'a>> {
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

    pub fn is_assignable_from(&'a self, other: &'a Class<'a>) -> bool {
        unimplemented!()
    }
    pub fn is_sub_class_of(&'a self, other: &'a Class<'a>) -> bool {
        unimplemented!()
    }
    pub fn is_implements(&'a self, iface: &'a Class<'a>) -> bool {
        unimplemented!()
    }
    pub fn is_sub_insterface_of(&'a self, iface: &'a Class<'a>) -> bool {
        unimplemented!()
    }
}

impl<'a> PartialEq for Class<'a> {
    fn eq(&self, other: &Class<'a>) -> bool {
        self.name == other.name // todo need to compare classloader
    }
}
