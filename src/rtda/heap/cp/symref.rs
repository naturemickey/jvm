// symbolic reference
struct SymRef {
    cp: Arc<ConstantPool>,
    class_name: String,
    option_class: Option<Arc<Class>>,
}

impl SymRef {
    pub fn new(cp: Arc<ConstantPool>, class_name: &str) -> SymRef {
        let class_name= class_name.to_string();
        Self { cp, class_name, option_class: None }
    }

    pub fn resoved_class(&mut self) -> Arc<Class> {
        match &self.option_class {
            Some(class) => class.clone(),
            None => self.resoved_class_ref()
        }
    }

    fn resoved_class_ref(&mut self) -> Arc<Class> {
//        let d = self.cp.class();
//        let c = d.lo
        unimplemented!()
    }
}