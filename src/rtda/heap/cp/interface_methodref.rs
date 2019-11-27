pub struct InterfaceMethodRef {
    member: MemberRef,
    method: Option<Arc<Method>>,
}

impl InterfaceMethodRef {
    fn new(info: &classfile::ConstantInterfaceMethodrefInfo, cp: Arc<RwLock<ConstantPool>>) -> InterfaceMethodRef {
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
        let d = self.member.cp().read().unwrap().class();
        let c = self.member.resoved_class();

        if !c.read().unwrap().is_interface() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let method = Self::lookup_interface_method(c, self.member.name(), self.member.descriptor());

        match method {
            Some(m) => {
                if !m.is_accessible_to(d) {
                    panic!("java.lang.IllegalAccessError");
                }
                m.clone()
            }
            None => {
                panic!("java.lang.NoSuchMethodError")
            }
        }
    }

    fn lookup_interface_method(iface: Arc<RwLock<Class>>, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        for method in &iface.read().unwrap().methods {
            if method.name() == name && method.descriptor() == descriptor {
                return Some(method.clone());
            }
        }

        MemberRef::lookup_method_in_interfaces(&iface.read().unwrap().interfaces, name, descriptor)
    }
}