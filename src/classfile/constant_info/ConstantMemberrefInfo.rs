pub struct ConstantMemberrefInfo {
    class_index: u16,
    name_and_type_index: u16,
    cp: Arc<RwLock<ConstantPool>>,
}

impl ConstantMemberrefInfo {
    fn new(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> ConstantMemberrefInfo {
        let class_index = reader.read_u16();
        let name_and_type_index = reader.read_u16();
        Self { class_index, name_and_type_index, cp: cp.clone() }
    }
    pub fn class_name(&self) -> Arc<String> {
        self.cp.read().unwrap().class_name(self.class_index)
    }
    pub fn name_and_descriptor(&self) -> (Arc<String>, Arc<String>) {
        self.cp.read().unwrap().name_and_type(self.name_and_type_index)
    }
}