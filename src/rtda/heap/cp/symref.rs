// symbolic reference
struct SymRef {
    cp: Arc<ConstantPool>,
    class_name: String,
    class: Option<Arc<Class>>,
}

impl SymRef {
    pub fn new(cp: Arc<ConstantPool>, class_name: &str) -> SymRef {
        let class_name = class_name.to_string();
        Self { cp, class_name, class: None }
    }

    pub fn resoved_class(&mut self) -> Arc<Class> {
        match &self.class {
            Some(c) => c.clone(),
            None => self.resoved_class_ref()
        }
    }

    fn resoved_class_ref(&mut self) -> Arc<Class> {
        let d = self.constant_pool().class();
        let c = ClassLoader::load_class(d.loader(), &self.class_name);
        //let c_class = unsafe { &*c };
        if !c.is_accessible_to(d.as_ref()) {
            panic!("java.lang.IllegalAccessError");
        }
        self.class = Some(c.clone());
        c.clone()
    }

    fn constant_pool(&self) -> Arc<ConstantPool> {
        self.cp.clone()
    }
}