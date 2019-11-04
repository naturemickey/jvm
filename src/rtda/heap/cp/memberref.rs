pub struct MemberRef {
    sym: SymRef,
    name: String,
    descriptor: String,
}

impl MemberRef {
    pub fn new(ref_info: &classfile::ConstantMemberrefInfo, cp: Arc<ConstantPool>) -> MemberRef {
        let class_name = ref_info.class_name();
        let (name, descriptor) = ref_info.name_and_descriptor();
        let sym = SymRef::new(cp, class_name);

        let (name, descriptor) = (name.to_string(), descriptor.to_string());
        Self { sym, name, descriptor }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    fn cp(&self) -> Arc<ConstantPool> {
        self.sym.cp.clone()
    }

    fn resoved_class(&mut self) -> Arc<Class> {
        self.sym.resoved_class()
    }
}