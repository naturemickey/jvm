pub struct InterfaceMethodRef {
    member: MemberRef,
    method: Option<Arc<Method>>,
}

impl InterfaceMethodRef {
    fn new(info: &classfile::ConstantInterfaceMethodrefInfo, cp: Arc<ConstantPool>) -> InterfaceMethodRef {
        let member = MemberRef::new(info.member(), cp);
        Self { member, method: None }
    }

    fn resolved_interface_method(&mut self) -> Arc<Method> {
        match &self.method {
            Some(arc_method) => arc_method.clone(),
            None => self.resolve_interface_method_ref()
        }
    }

    fn resolve_interface_method_ref(&mut self) -> Arc<Method> {
        unimplemented!()
    }
}