pub struct Field<'a> {
    member: ClassMember<'a>,
    const_value_index: usize,
    slot_id: usize,
}

impl<'a> Field<'a> {
    pub fn new_fields() -> Vec<Field<'a>> {
        unimplemented!()
    }


    fn is_public(&self) -> bool {
        self.member.is_public()
    }
    fn is_private(&self) -> bool {
        self.member.is_private()
    }
    fn is_protected(&self) -> bool {
        self.member.is_protected()
    }
    fn is_static(&self) -> bool {
        self.member.is_static()
    }
    fn is_synthetic(&self) -> bool {
        self.member.is_synthetic()
    }
    fn name(&self) -> &str {
        self.member.name()
    }
    fn descriptor(&self) -> &str {
        self.member.descriptor()
    }
    fn class(&'a self) -> &'a Class<'a> {
        self.member.class()
    }
    fn is_accessible_to(&'a self, d: &'a Class<'a>) -> bool {
        self.member.is_accessible_to(d)
    }
}