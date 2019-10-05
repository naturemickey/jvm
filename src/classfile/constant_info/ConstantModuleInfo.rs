struct ConstantModuleInfo {
    name_index: u16
}

impl ConstantModuleInfo {
    fn new(reader: &mut ClassReader) -> ConstantModuleInfo {
        Self { name_index: reader.read_u16() }
    }
}