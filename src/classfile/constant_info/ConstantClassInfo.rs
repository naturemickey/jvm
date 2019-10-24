pub struct ConstantClassInfo {
    string_info: ConstantStringInfo,
}

impl ConstantClassInfo {
    fn new(reader: &mut ClassReader) -> ConstantClassInfo {
        Self { string_info: ConstantStringInfo::new(reader) }
    }
    fn name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        self.string_info.string(cp)
    }
}