pub struct FieldRef {
    member: MemberRef,
    field: Option<*const Field>,
}

impl FieldRef {
    fn new(info: &classfile::ConstantFieldrefInfo, cp: *const ConstantPool) -> FieldRef {
        let member = MemberRef::new(info.member(), cp);
        Self { member, field: None }
    }

    fn resolved_field(&mut self) -> *const Field {
        match self.field {
            Some(f) => f,
            None => self.resolve_field_ref()
        }
    }

    fn resolve_field_ref(&mut self) -> *const Field {
        let c = self.member.resoved_class();
        let name = self.member.name();
        let descriptor = self.member.descriptor();

        let field = Self::lookup_field(c, name, descriptor);

        match field {
            Some(field) => {
                let d = self.member.cp().class();
                if !unsafe { &*field }.is_accessible_to(d) {
                    panic!("java.lang.IllegalAccessError");
                }
                self.field = Some(field);
                field
            }
            None => {
                panic!("java.lang.NoSuchFieldError")
            }
        }
    }

    fn lookup_field(c: *const Class, name: &str, descriptor: &str) -> Option<*const Field> {
        let class = unsafe { &*c };
        for field in &class.fields {
            if field.name().eq(name) && field.descriptor().eq(descriptor) {
                return Some(field);
            }
        }

        for iface in &class.interfaces {
            let field = Self::lookup_field(*iface, name, descriptor);
            if field != None {
                return field;
            }
        }

        match &class.super_class {
            Some(cptr) => Self::lookup_field(*cptr, name, descriptor),
            None => None
        }
    }
}