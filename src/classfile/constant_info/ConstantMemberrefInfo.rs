pub struct ConstantMemberrefInfo {
    class_index: u16,
    name_and_type_index: u16,
    cp: Arc<ConstantPool>,
}

impl ConstantMemberrefInfo {
    fn new(reader: &mut ClassReader, cp: Arc<ConstantPool>) -> ConstantMemberrefInfo {
        let class_index = reader.read_u16();
        let name_and_type_index = reader.read_u16();
        Self { class_index, name_and_type_index, cp: cp.clone() }
    }
    fn class_name<'a>(&'a self) -> &'a str {
        self.cp.class_name(self.class_index)
    }
    fn name_and_descriptor<'a>(&'a self) -> (&'a str, &'a str) {
        self.cp.name_and_type(self.name_and_type_index)
    }
}