pub struct Field {
    member: ClassMember,
    const_value_index: u16,
    slot_id: usize,
}

impl Field {
    pub fn new_fields(class: Arc<RwLock<Class>>, cf_fields: &Vec<MemberInfo>) -> Vec<Arc<RwLock<Field>>> {
        let mut fields = Vec::with_capacity(cf_fields.len());
        for cf_field in cf_fields {
            let member = ClassMember::new(class.clone(), cf_field);
            let field = match cf_field.constant_value_attribute() {
                Some(cva) => Self { member, const_value_index: cva.value_index(), slot_id: 0 },
                None => Self { member, const_value_index: 0, slot_id: 0 }
            };
            fields.push(Arc::new(RwLock::new(field)));
        }

        fields
    }

    pub fn const_value_index(&self) -> u16 {
        self.const_value_index
    }
    pub fn slot_id(&self) -> usize {
        self.slot_id
    }
    pub fn is_long_or_double(&self) -> bool {
        self.descriptor() == "J" || self.descriptor() == "D"
    }

    pub fn is_volatile(&self) -> bool {
        self.member.access_flags & ACC_VOLATILE != 0
    }
    pub fn is_transient(&self) -> bool {
        self.member.access_flags & ACC_TRANSIENT != 0
    }
    pub fn is_enum(&self) -> bool {
        self.member.access_flags & ACC_ENUM != 0
    }

    pub fn is_public(&self) -> bool {
        self.member.is_public()
    }
    pub fn is_private(&self) -> bool {
        self.member.is_private()
    }
    pub fn is_protected(&self) -> bool {
        self.member.is_protected()
    }
    pub fn is_static(&self) -> bool {
        self.member.is_static()
    }
    pub fn is_synthetic(&self) -> bool {
        self.member.is_synthetic()
    }
    pub fn is_final(&self) -> bool { self.member.is_final() }

    pub fn name(&self) -> &str {
        self.member.name()
    }
    pub fn descriptor(&self) -> &str {
        self.member.descriptor()
    }
    pub fn class(&self) -> Arc<RwLock<Class>> {
        self.member.class()
    }
    fn is_accessible_to(&self, d: Arc<RwLock<Class>>) -> bool {
        self.member.is_accessible_to(d)
    }
}