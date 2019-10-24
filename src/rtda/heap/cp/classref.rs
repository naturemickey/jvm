pub struct ClassRef<'a> {
    sym: SymRef<'a>
}

impl<'a> ClassRef<'a> {
    fn new(cp: Arc<ConstantPool<'a>>, class_info: &'a classfile::ConstantClassInfo) -> ClassRef<'a> {
        let class_name = class_info.name();
        let sym = SymRef::new(cp.clone(), class_name);
        Self { sym }
    }
}
