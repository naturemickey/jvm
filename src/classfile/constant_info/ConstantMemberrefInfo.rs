pub struct ConstantMemberrefInfo {
    class_index: u16,
    name_and_type_index: u16,
}

impl ConstantMemberrefInfo {
    fn new(reader: &mut ClassReader) -> ConstantMemberrefInfo {
        let class_index = reader.read_u16();
        let name_and_type_index = reader.read_u16();
        Self { class_index, name_and_type_index }
    }
    fn class_name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        cp.class_name(self.class_index)
    }
    fn name_and_descriptor<'a>(&'a self, cp: &'a ConstantPool) -> (&'a str, &'a str) {
        cp.name_and_type(self.name_and_type_index)
    }
}