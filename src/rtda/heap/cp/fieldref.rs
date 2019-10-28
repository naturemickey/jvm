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

    fn lookup_field(&mut self) -> Option<Arc<Field>> {
         let c = {self.member.resoved_class().clone()};
        let name = self.member.name();
//        let descriptor = self.member.descriptor();
//        panic!("todo")
        unimplemented!()
    }
}