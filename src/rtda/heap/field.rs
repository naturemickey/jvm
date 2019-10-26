pub struct Field<'a> {
    member: ClassMember<'a>,
    const_value_index: u16,
    slot_id: usize,
}

impl<'a> Field<'a> {
    pub fn new_fields(arc_class: Arc<Class<'a>>, cf_fields: &'a Vec<MemberInfo>) -> Vec<Field<'a>> {
        let mut fields = Vec::with_capacity(cf_fields.len());
        for cf_field in cf_fields {
            let member = ClassMember::new(arc_class.clone(), cf_field);
            fields.push(match cf_field.constant_value_attribute() {
                Some(cva) => Self { member, const_value_index: cva.value_index(), slot_id: 0 },
                None => Self { member, const_value_index: 0, slot_id: 0 }
            });
        }

        fields
    }

    fn const_value_index(&self) -> u16 {
        self.const_value_index
    }
    fn slot_id(&self) -> usize {
        self.slot_id
    }
    fn is_long_or_double(&self) -> bool {
        self.descriptor() == "J" || self.descriptor() == "D"
    }

    fn is_volatile(&self) -> bool {
        self.member.access_flags & ACC_VOLATILE != 0
    }
    fn is_transient(&self) -> bool {
        self.member.access_flags & ACC_TRANSIENT != 0
    }
    fn is_enum(&self) -> bool {
        self.member.access_flags & ACC_ENUM != 0
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
    fn is_accessible_to2(&'a self, d: Arc<Class<'a>>) -> bool {
        self.member.is_accessible_to(d.borrow())
    }
}