struct FieldRef {
    member: MemberRef,
    field: Option<Arc<Field>>,
}

impl FieldRef {
    fn new(ref_info: &classfile::ConstantFieldrefInfo, cp: Arc<ConstantPool>) -> FieldRef {
        let member = MemberRef::new(ref_info.member(), cp.clone());
        Self { member, field: None }
    }

    fn resolved_field(&mut self) -> Arc<Field> {
        match &self.field {
            Some(f) => f.clone(),
            None => self.resolve_field_ref()
        }
    }

    fn resolve_field_ref(&mut self) -> Arc<Field> {
        let c = self.member.resoved_class();
        let name = self.member.name();
        let descriptor = self.member.descriptor();

        let field = self.lookup_field(c, name, descriptor);

        match &field {
            Some(arc_field) => {
                let d = self.member.cp().class();
                if !arc_field.is_accessible_to2(d) {
                    panic!("java.lang.IllegalAccessError");
                }
                self.field = Some(arc_field.clone());
                arc_field.clone()
            }
            None => {
                panic!("java.lang.NoSuchFieldError")
            }
        }
    }

    fn lookup_field(&self, c: Arc<Class>, name: &str, descriptor: &str) -> Option<Arc<Field>> {
        for field in &c.fields {
            if field.name().eq(name) && field.descriptor().eq(descriptor) {
                return Some(field.clone());
            }
        }

        // todo for c.interfaces

        // todo c.super_class

        None
    }
}