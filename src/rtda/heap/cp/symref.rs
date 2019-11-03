// symbolic reference
struct SymRef {
    cp: *const ConstantPool,
    class_name: String,
    class: Option<*const Class>,
}

impl SymRef {
    pub fn new(cp: *const ConstantPool, class_name: &str) -> SymRef {
        let class_name = class_name.to_string();
        Self { cp, class_name, class: None }
    }

    pub fn resoved_class(&mut self) -> *const Class {
        match self.class {
            Some(c) => unsafe { &*c },
            None => self.resoved_class_ref()
        }
    }

    fn resoved_class_ref(&mut self) -> *const Class {
        let d = self.constant_pool().class_mut();
        let c = d.loader_mut().load_class(&self.class_name);
        let c_class = unsafe { &*c };
        if !c_class.is_accessible_to(d) {
            panic!("java.lang.IllegalAccessError");
        }
        self.class = Some(c);
        c
    }

    fn constant_pool(&self) -> &ConstantPool {
        unsafe { &*self.cp }
    }
}