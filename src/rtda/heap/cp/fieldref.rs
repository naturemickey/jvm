pub struct FieldRef {
    member: MemberRef,
    field: Option<Arc<RwLock<Field>>>,
}

impl FieldRef {
    fn new(info: &classfile::ConstantFieldrefInfo, cp: Arc<RwLock<ConstantPool>>) -> FieldRef {
        let member = MemberRef::new(info.member(), cp);
        Self { member, field: None }
    }

    pub fn resolved_field(&mut self) -> Arc<RwLock<Field>> {
        match &self.field {
            Some(f) => f.clone(),
            None => self.resolve_field_ref()
        }
    }

    fn resolve_field_ref(&mut self) -> Arc<RwLock<Field>> {
        let c = self.member.resoved_class();
        let name = self.member.name();
        let descriptor = self.member.descriptor();

        let field = Self::lookup_field(c, name, descriptor);

        match field {
            Some(field) => {
                let d = self.member.cp().read().unwrap().class();
                if !field.read().unwrap().is_accessible_to(d) {
                    panic!("java.lang.IllegalAccessError");
                }
                self.field = Some(field.clone());
                field.clone()
            }
            None => {
                panic!("java.lang.NoSuchFieldError")
            }
        }
    }

    fn lookup_field(class: Arc<RwLock<Class>>, name: &str, descriptor: &str) -> Option<Arc<RwLock<Field>>> {
        for field in &class.read().unwrap().fields {
            if field.read().unwrap().name().eq(name) && field.read().unwrap().descriptor().eq(descriptor) {
                return Some(field.clone());
            }
        }

        for iface in &class.read().unwrap().interfaces {
            let field = Self::lookup_field(iface.clone(), name, descriptor);
            match field {
                Some(f) => return Some(f),
                None => {}
            }
        }

        match &class.read().unwrap().super_class {
            Some(cptr) => Self::lookup_field(cptr.clone(), name, descriptor),
            None => None
        }
    }
}