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
//        unimplemented!()
        let field = self.lookup_field();

        match field {
            Some(arc_field) => {
                //self.field = Some(arc_field.clone());
                arc_field.clone()
            },
            None => {
                panic!("java.lang.NoSuchFieldError")
            }
        }
//
//        let arc_field = match &field {
//            Some(arc_field) => {
//                let cp = self.member.cp();
//                let d = cp.class();
//                if arc_field.is_accessible_to2(d) {
//                    panic!("java.lang.IllegalAccessError");
//                }
//                arc_field.clone()
//            }
//            None => panic!("java.lang.NoSuchFieldError")
//        };
//
//        self.field = field;
//
//        arc_field.clone()
    }

    fn lookup_field(&'a mut self) -> Option<Arc<Field<'a>>> {
         let c = {self.member.resoved_class().clone()};
        let name = self.member.name();
//        let descriptor = self.member.descriptor();
//        panic!("todo")
        unimplemented!()
    }
}