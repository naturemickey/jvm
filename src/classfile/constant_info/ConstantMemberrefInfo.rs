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
    pub fn class_name(&self) -> &str {
        self.cp.class_name(self.class_index)
    }
    pub fn name_and_descriptor(&self) -> (&str, &str) {
        self.cp.name_and_type(self.name_and_type_index)
    }
}