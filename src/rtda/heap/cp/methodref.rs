pub struct MethodRef {
    member: MemberRef,
    method: Option<Arc<Method>>,
}

impl MethodRef {
    fn new(info: &classfile::ConstantMethodrefInfo, cp: Arc<RwLock<ConstantPool>>) -> MethodRef {
        let member = MemberRef::new(info.member(), cp);
        Self { member, method: None }
    }

    fn resolved_method(&mut self) -> Arc<Method> {
        match &self.method {
            Some(arc_method) => arc_method.clone(),
            None => self.resolve_method_ref()
        }
    }

    pub fn resolve_method_ref(&mut self) -> Arc<Method> {
        let d = self.member.cp().read().unwrap().class();
        let c = self.member.resoved_class();

        if c.read().unwrap().is_interface() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let method = Self::lookup_method(c.clone(), self.name(), self.descriptor());

        match method {
            Some(m) => {
                if !m.is_accessible_to(d) {
                    panic!("java.lang.IllegalAccessError");
                }
                self.method = Some(m.clone());

                m.clone()
            }
            None => {
                panic!("java.lang.NoSuchMethodError")
            }
        }
    }

    fn lookup_method(class: Arc<RwLock<Class>>, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        let method = MemberRef::lookup_method_in_class(class.clone(), name, descriptor);
        if method.is_none() {
            MemberRef::lookup_method_in_interfaces(&class.read().unwrap().interfaces, name, descriptor)
        } else {
            method
        }
    }

    pub fn name(&self) -> &str {
        self.member.name()
    }
    pub fn descriptor(&self) -> &str { self.member.descriptor() }
}