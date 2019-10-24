pub struct ConstantFieldrefInfo {
    member: ConstantMemberrefInfo,
}

impl ConstantFieldrefInfo {
    fn new(reader: &mut ClassReader, cp: Arc<ConstantPool>) -> ConstantFieldrefInfo {
        Self { member: ConstantMemberrefInfo::new(reader, cp.clone()) }
    }
    fn class_name<'a>(&'a self) -> &'a str {
        self.member.class_name()
    }
    fn name_and_descriptor<'a>(&'a self) -> (&'a str, &'a str) {
        self.member.name_and_descriptor()
    }
}