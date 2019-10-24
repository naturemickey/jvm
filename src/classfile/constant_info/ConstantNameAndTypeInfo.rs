pub struct ConstantNameAndTypeInfo {
    name_index: u16,
    descriptor_index: u16,
}

impl ConstantNameAndTypeInfo {
    fn new(reader: &mut ClassReader) -> ConstantNameAndTypeInfo {
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        Self { name_index, descriptor_index }
    }
}