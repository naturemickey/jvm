struct ConstantMethodTypeInfo {
    descriptor_index: u16
}

impl ConstantMethodTypeInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodTypeInfo {
        Self { descriptor_index: reader.read_u16() }
    }
}