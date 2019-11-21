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
        let d = self.cp().class();
        let c = self.member.resoved_class();

        if c.is_interface() {
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

    fn lookup_method(class: Arc<Class>, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        let method = Self::lookup_method_in_class(class.clone(), name, descriptor);
        if method.is_none() {
            Self::lookup_method_in_interfaces(&class.interfaces, name, descriptor)
        } else {
            method
        }
    }

    fn lookup_method_in_class(class: Arc<Class>, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        for method in &class.methods {
            if method.name() == name && method.descriptor() == descriptor {
                return Some(method.clone());
            }
        }
        match &class.super_class {
            Some(c) => Self::lookup_method_in_class(c.clone(), name, descriptor),
            None => None,
        }
    }

    fn lookup_method_in_interfaces(ifaces: &Vec<Arc<Class>>, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        for iface in ifaces {
            for method in &iface.methods {
                if method.name() == name && method.descriptor() == descriptor {
                    return Some(method.clone());
                }
            }

            let method = Self::lookup_method_in_interfaces(&iface.interfaces, name, descriptor);
            if method.is_some() {
                return method;
            }
        }

        None
    }

    pub fn name(&self) -> &str {
        self.member.name()
    }
    pub fn descriptor(&self) -> &str { self.member.descriptor() }
    fn cp(&self) -> Arc<ConstantPool> { self.member.cp() }
}