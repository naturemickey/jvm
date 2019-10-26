struct FieldRef<'a> {
    member: MemberRef<'a>,
    field: Option<Arc<Field<'a>>>,
}

impl<'a> FieldRef<'a> {
    fn new(ref_info: &'a classfile::ConstantFieldrefInfo, cp: Arc<ConstantPool<'a>>) -> FieldRef<'a> {
        let member = MemberRef::new(ref_info.member(), cp.clone());
        Self { member, field: None }
    }

    fn resolved_field(&'a mut self) -> Arc<Field<'a>> {
        match &self.field {
            Some(f) => f.clone(),
            None => self.resolve_field_ref()
        }
    }

    fn resolve_field_ref(&'a mut self) -> Arc<Field<'a>> {
        let member:&'a mut MemberRef<'a> = &mut self.member;
        let cp = member.cp();
        let arc_d = cp.class();
        let d = arc_d.borrow();

        let field = self.lookup_field(member.resoved_class(), member.name(), member.descriptor());

        let res = match &field {
            Some(arc_field) => {
                if arc_field.is_accessible_to(d) {
                    panic!("java.lang.IllegalAccessError");
                }
                arc_field.clone()
            }
            None => panic!("java.lang.NoSuchFieldError")
        };

        self.field = field;

        res.clone()
    }

    fn lookup_field(&'a mut self, c: &'a Class<'a>, name: &'a str, descriptor: &'a str) -> Option<Arc<Field<'a>>> {
        unimplemented!()
    }
}