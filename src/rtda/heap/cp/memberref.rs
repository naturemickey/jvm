pub struct MemberRef<'a> {
    sym: SymRef<'a>,
    name: &'a str,
    descriptor: &'a str,
}

impl<'a> MemberRef<'a> {
    pub fn new(ref_info: &'a classfile::ConstantMemberrefInfo, cp: Arc<ConstantPool<'a>>) -> MemberRef<'a> {
        let class_name = ref_info.class_name();
        let (name, descriptor) = ref_info.name_and_descriptor();
        let sym = SymRef::new(cp.clone(), class_name);
        Self { sym, name, descriptor }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn descriptor(&self) -> &str {
        self.descriptor
    }

    fn cp(&'a self) -> Arc<ConstantPool<'a>> {
        self.sym.cp.clone()
    }

    fn resoved_class(&'a mut self) -> &'a Class<'a> {
        self.sym.resoved_class()
    }
}