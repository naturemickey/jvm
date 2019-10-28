struct MethodRef {
    member: MemberRef,
    method: Option<Arc<Method>>,
}

impl MethodRef {
    fn new(cp: Arc<ConstantPool>, ref_info: &classfile::ConstantMethodrefInfo) -> MethodRef {
        let member = MemberRef::new(ref_info.member(), cp);
        Self { member, method: None }
    }

    fn resolved_method(&mut self) -> Arc<Method> {
        match &self.method {
            Some(arc_method) => arc_method.clone(),
            None => self.resolve_method_ref()
        }
    }

    fn resolve_method_ref(&mut self) -> Arc<Method> {
        unimplemented!()
    }
}