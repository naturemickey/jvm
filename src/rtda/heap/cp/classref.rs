pub struct ClassRef {
    sym: SymRef
}

impl ClassRef {
    fn new(cp: Arc<ConstantPool>, class_info: &classfile::ConstantClassInfo) -> ClassRef {
        let class_name = class_info.name();
        let sym = SymRef::new(cp.clone(), class_name);
        Self { sym }
    }
}
