pub struct ConstantMethodrefInfo {
    member: ConstantMemberrefInfo,
}

impl ConstantMethodrefInfo {
    fn new(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> ConstantMethodrefInfo {
        Self { member: ConstantMemberrefInfo::new(reader, cp.clone()) }
    }
    pub fn class_name(&self) -> Arc<String> {
        self.member.class_name()
    }
    pub fn name_and_descriptor(&self) -> (Arc<String>, Arc<String>) {
        self.member.name_and_descriptor()
    }
    pub fn member(&self) -> &ConstantMemberrefInfo {
        &self.member
    }
}