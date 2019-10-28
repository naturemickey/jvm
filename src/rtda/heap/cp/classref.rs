pub struct ClassRef {
    sym: SymRef
}

impl ClassRef {
    fn new(class_info: &classfile::ConstantClassInfo, cp: *const ConstantPool) -> ClassRef {
        let class_name = class_info.name();
        let sym = SymRef::new(cp, class_name);
        Self { sym }
    }
}
