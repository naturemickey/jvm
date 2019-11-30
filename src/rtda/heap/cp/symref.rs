// symbolic reference
struct SymRef {
    cp: Arc<RwLock<ConstantPool>>,
    class_name: Arc<String>,
    class: Option<Arc<RwLock<Class>>>,
}

impl SymRef {
    pub fn new(cp: Arc<RwLock<ConstantPool>>, class_name: Arc<String>) -> SymRef {
        Self { cp, class_name, class: None }
    }

    pub fn resoved_class(&mut self) -> Arc<RwLock<Class>> {
        match &self.class {
            Some(c) => c.clone(),
            None => self.resoved_class_ref()
        }
    }

    fn resoved_class_ref(&mut self) -> Arc<RwLock<Class>> {
        let d = self.constant_pool().read().unwrap().class();
        let c = ClassLoader::load_class(d.read().unwrap().loader(), &self.class_name);

        if !c.read().unwrap().is_accessible_to(d.read().unwrap().deref()) {
            panic!("java.lang.IllegalAccessError");
        }
        self.class = Some(c.clone());
        c.clone()
    }

    fn constant_pool(&self) -> Arc<RwLock<ConstantPool>> {
        self.cp.clone()
    }
}