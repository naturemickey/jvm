// symbolic reference
struct SymRef {
    cp: *const ConstantPool,
    class_name: String,
    option_class: Option<*const Class>,
}

impl SymRef {
    pub fn new(cp: *const ConstantPool, class_name: &str) -> SymRef {
        let class_name = class_name.to_string();
        Self { cp, class_name, option_class: None }
    }

    pub fn resoved_class(&mut self) -> *const Class {
        match self.option_class {
            Some(class) => unsafe { &*class },
            None => self.resoved_class_ref()
        }
    }

    fn resoved_class_ref(&mut self) -> *const Class {
//        let d = self.cp.class();
//        let c = d.lo
        unimplemented!()
    }
}