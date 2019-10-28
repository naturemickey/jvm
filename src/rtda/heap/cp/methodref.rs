struct MethodRef {
    member: MemberRef,
    method: Option<Arc<Method>>,
}

impl MethodRef {
    fn new(cp: Arc<ConstantPool>, ref_info: &classfile::ConstantMethodrefInfo) -> MethodRef {
        let member = MemberRef::new(ref_info.member(), cp);
        Self { member, method: None }
    }

    
}