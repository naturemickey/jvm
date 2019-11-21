pub struct MethodRef {
    member: MemberRef,
    method: Option<Arc<Method>>,
}

impl MethodRef {
    fn new(info: &classfile::ConstantMethodrefInfo, cp: Arc<ConstantPool>) -> MethodRef {
        let member = MemberRef::new(info.member(), cp);
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

    fn lookup_method(class:Arc<Class>, name:&str, descriptor:&str)-> Arc<Method> {
        unimplemented!()
    }

    fn lookup_method_in_class(class:Arc<Class>, name:&str, descriptor:&str) -> Arc<Method>{
        unimplemented!()
    }

    fn lookup_method_in_interfaces(ifaces:Vec<Arc<Class>>, name:&str, descriptor:&str) -> Arc<Method> {
        unimplemented!()
    }

    pub fn name(&self) -> &str {
        self.member.name()
    }
    pub fn descriptor(&self) -> &str { self.member.descriptor() }
}