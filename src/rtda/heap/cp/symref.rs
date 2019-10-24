// symbolic reference
struct SymRef<'a> {
    cp: Arc<ConstantPool<'a>>,
    class_name: &'a str,
    option_class: Option<&'a Class<'a>>,
}

impl<'a> SymRef<'a> {
    pub fn new(cp: Arc<ConstantPool<'a>>, class_name: &'a str) -> SymRef<'a> {
        Self { cp, class_name, option_class: None }
    }

    pub fn resoved_class(&mut self) -> &'a Class<'a> {
        match self.option_class {
            Some(class) => class,
            None => self.resoved_class_ref()
        }
    }

    fn resoved_class_ref(&mut self) -> &'a Class<'a> {
        unimplemented!()
    }
}