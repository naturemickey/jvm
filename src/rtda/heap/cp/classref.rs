pub struct ClassRef {
    sym: SymRef
}

impl ClassRef {
    fn new(info: &classfile::ConstantClassInfo, cp: Arc<RwLock<ConstantPool>>) -> ClassRef {
        let class_name = info.name();
        let sym = SymRef::new(cp, class_name);
        Self { sym }
    }

    pub fn resolved_class(&mut self) -> Arc<RwLock<Class>> {
        self.sym.resoved_class()
    }
}
