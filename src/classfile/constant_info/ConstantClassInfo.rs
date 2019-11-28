pub struct ConstantClassInfo {
    string_info: ConstantStringInfo
}

impl ConstantClassInfo {
    fn new(reader: &mut ClassReader, cp:Arc<RwLock<ConstantPool>>) -> ConstantClassInfo {
        Self { string_info: ConstantStringInfo::new(reader, cp.clone()) }
    }
    pub fn name(&self) -> Arc<String> {
        self.string_info.string()
    }
}