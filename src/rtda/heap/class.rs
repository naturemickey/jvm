pub struct Class {
    access_flags: u16,
    name: String,
    super_class_name: String,
    interface_names: Vec<Arc<String>>,
    constant_pool: Arc<RwLock<ConstantPool>>,
    fields: Vec<Arc<RwLock<Field>>>,
    methods: Vec<Arc<Method>>,
    loader: Arc<RwLock<ClassLoader>>,
    super_class: Option<Arc<RwLock<Class>>>,
    interfaces: Vec<Arc<RwLock<Class>>>,
    instance_slot_count: usize,
    static_slot_count: usize,
    static_vars: Slots,
}

impl Class {
    pub fn new(cf: &ClassFile, loader: Arc<RwLock<ClassLoader>>) -> Arc<RwLock<Class>> {
        let access_flags = cf.access_flags();
        let name = cf.class_name().to_string();
        let super_class_name = dbg!(cf.super_class_name().to_string());
        let interface_names = cf.interface_names();
        let constant_pool = ConstantPool::new(cf.constant_pool(), None);
        let fields = Vec::with_capacity(0);
        let methods = Vec::with_capacity(0);
        let static_vars = Slots::new(0);

        let class =
            Arc::new(RwLock::new(Self {
                access_flags,
                name,
                super_class_name,
                interface_names,
                constant_pool: constant_pool.clone(),
                fields,
                methods,
                loader,
                super_class: None,
                interfaces: Vec::new(),
                instance_slot_count: 0,
                static_slot_count: 0,
                static_vars,
            }));

        constant_pool.write().unwrap().set_class(Some(class.clone()));

        let methods = Method::new_methods(class.clone(), cf.methods());
        let fields = Field::new_fields(class.clone(), cf.fields());

        // let class: &mut Self = Arc::get_mut(&mut arc_class).unwrap();
        class.write().unwrap().methods = methods;
        class.write().unwrap().fields = fields;

        class
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

    pub fn constant_pool(&self) -> Arc<RwLock<ConstantPool>> {
        self.constant_pool.clone()
    }
    pub fn static_vars(&self) -> &Slots {
        &self.static_vars
    }
    pub fn static_vars_mut(&mut self) -> &mut Slots {
        &mut self.static_vars
    }

    pub fn is_accessible_to(&self, other: &Class) -> bool {
        self.is_public() || self.package_name().eq(other.package_name())
    }
    pub fn package_name(&self) -> &str {
        match self.name.rfind("/") {
            Some(i) => {
                self.name.get(..i).unwrap()
            }
            None => "",
        }
    }

    pub fn get_main_method(&self) -> Option<Arc<Method>> {
        self.get_static_method("main", "([Ljava/lang/String;)V")
    }

    pub fn get_static_method(&self, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        for method in &self.methods {
            if method.is_static() && method.name() == name && method.descriptor() == descriptor {
                return Some(method.clone());
            }
        }
        None
    }

    pub fn new_object(class: Arc<RwLock<Class>>) -> Arc<RwLock<Object>> {
        Object::new(class)
    }

    pub fn is_assignable_from(&self, other: &Class) -> bool {
        let (s, t) = (other, self);
        if s == t {
            true
        } else {
            if t.is_interface() {
                return s.is_implements(t);
            } else {
                return s.is_sub_class_of(t);
            }
        }
    }
    pub fn is_sub_class_of(&self, other: &Class) -> bool {
        match &self.super_class {
            None => false,
            Some(c) => {
                let c = c.read().unwrap();
                c.deref() == other || c.deref().is_sub_class_of(other)
            }
        }
//        let c = self.super_class;
//        while c != None {
//            let c = c.unwrap().borrow();
//            if c == other {
//                return true;
//            }
//            let c = c.super_class;
//        }
//        return false;
    }
    pub fn is_implements(&self, iface: &Class) -> bool {
        for interface in &self.interfaces {
            let inf = interface.read().unwrap();
            let interface = inf.deref();
            if interface == iface || interface.is_sub_insterface_of(iface) {
                return true;
            }
        }
        match &self.super_class {
            None => false,
            Some(c) => c.read().unwrap().is_implements(iface)
        }
    }
    pub fn is_sub_insterface_of(&self, iface: &Class) -> bool {
        for super_interface in &self.interfaces {
            let si = super_interface.read().unwrap();
            let super_interface = si.deref();
            if super_interface == iface || super_interface.is_sub_insterface_of(iface) {
                return true;
            }
        }
        return false;
    }

    fn loader(&self) -> Arc<RwLock<ClassLoader>> {
        self.loader.clone()
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Class) -> bool {
        self as *const Class == other as *const Class
        // self.name == other.name // todo need to compare classloader
    }
}
