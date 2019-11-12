pub struct FieldRef {
    member: MemberRef,
    field: Option<Arc<Field>>,
}

impl FieldRef {
    fn new(info: &classfile::ConstantFieldrefInfo, cp: Arc<ConstantPool>) -> FieldRef {
        let member = MemberRef::new(info.member(), cp);
        Self { member, field: None }
    }

    pub fn resolved_field(&mut self) -> Arc<Field> {
        match &self.field {
            Some(f) => f.clone(),
            None => self.resolve_field_ref()
        }
    }

    fn resolve_field_ref(&mut self) -> Arc<Field> {
        let c = self.member.resoved_class();
        let name = self.member.name();
        let descriptor = self.member.descriptor();

        let field = Self::lookup_field(c, name, descriptor);

        match field {
            Some(field) => {
                let d = self.member.cp().class();
                if !field.is_accessible_to(d) {
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

    fn lookup_field(class: Arc<Class>, name: &str, descriptor: &str) -> Option<Arc<Field>> {
        for field in &class.fields {
            if field.name().eq(name) && field.descriptor().eq(descriptor) {
                return Some(field.clone());
            }
        }

        for iface in &class.interfaces {
            let field = Self::lookup_field(iface.clone(), name, descriptor);
            match field {
                Some(f) => return Some(f),
                None => {}
            }
        }

        match &class.super_class {
            Some(cptr) => Self::lookup_field(cptr.clone(), name, descriptor),
            None => None
        }
    }
}