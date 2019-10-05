struct ConstantInvokeDynamicInfo {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
}

impl ConstantInvokeDynamicInfo {
    fn new(reader: &mut ClassReader) -> ConstantInvokeDynamicInfo {
        let bootstrap_method_attr_index = reader.read_u16();
        let name_and_type_index = reader.read_u16();
        Self { bootstrap_method_attr_index, name_and_type_index }
    }
}