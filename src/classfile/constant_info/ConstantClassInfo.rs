pub struct ConstantClassInfo {
    string_info: ConstantStringInfo
}

impl ConstantClassInfo {
    fn new(reader: &mut ClassReader, cp:Arc<ConstantPool>) -> ConstantClassInfo {
        Self { string_info: ConstantStringInfo::new(reader, cp.clone()) }
    }
    pub fn name<'a>(&'a self) -> &'a str {
        self.string_info.string()
    }
}