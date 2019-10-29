pub struct ClassRef {
    sym: SymRef
}

impl ClassRef {
    fn new(info: &classfile::ConstantClassInfo, cp: *const ConstantPool) -> ClassRef {
        let class_name = info.name();
        let sym = SymRef::new(cp, class_name);
        Self { sym }
    }
}
