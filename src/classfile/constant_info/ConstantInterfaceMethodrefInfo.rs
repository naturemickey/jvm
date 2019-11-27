pub struct ConstantInterfaceMethodrefInfo {
    member: ConstantMemberrefInfo,
}

impl ConstantInterfaceMethodrefInfo {
    fn new(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> ConstantInterfaceMethodrefInfo {
        Self { member: ConstantMemberrefInfo::new(reader, cp.clone()) }
    }
    fn class_name(&self) -> &str {
        self.member.class_name()
    }
    fn name_and_descriptor(&self) -> (&str, &str) {
        self.member.name_and_descriptor()
    }
    pub fn member(&self) -> &ConstantMemberrefInfo {
        &self.member
    }
}