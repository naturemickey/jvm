// symbolic reference
struct SymRef<'a> {
    cp: Arc<ConstantPool<'a>>,
    class_name: &'a str,
    option_class: Option<Arc<Class<'a>>>,
}

impl<'a> SymRef<'a> {
    pub fn new(cp: Arc<ConstantPool<'a>>, class_name: &'a str) -> SymRef<'a> {
        Self { cp, class_name, option_class: None }
    }

    pub fn resoved_class(&'a mut self) -> Arc<Class<'a>> {
        match &self.option_class {
            Some(class) => class.clone(),
            None => self.resoved_class_ref()
        }
    }

    fn resoved_class_ref(&'a mut self) -> Arc<Class<'a>> {
//        let d = self.cp.class();
//        let c = d.lo
        unimplemented!()
    }
}