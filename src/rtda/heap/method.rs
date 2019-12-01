pub struct Method {
    member: ClassMember,
    max_stack: u16,
    max_locals: u16,
    code: Arc<Vec<u8>>,
    arg_slot_count: usize,
}

impl Method {
    pub fn new_methods(class: Arc<RwLock<Class>>, cf_methods: &Vec<MemberInfo>) -> Vec<Arc<Method>> {
        let mut methods = Vec::with_capacity(cf_methods.len());
        for cf_method in cf_methods {
            let member = ClassMember::new(class.clone(), cf_method);
            let (max_stack, max_locals, code) = Self::copy_attributes(cf_method);
            let arg_slot_count = Self::calc_arg_slot_count(&member);
            methods.push(Arc::new(Self { member, max_stack, max_locals, code, arg_slot_count }));
        }
        methods
    }

    fn copy_attributes(cf_method: &MemberInfo) -> (u16, u16, Arc<Vec<u8>>) {
        match cf_method.code_attribute() {
            Some(code_attr) => {
                let max_stack = code_attr.max_stack();
                let max_locals = code_attr.max_locals();
                let code = code_attr.code();
                (max_stack, max_locals, code)
            }
            None => (0, 0, Arc::new(Vec::with_capacity(0)))
        }
    }

    pub fn is_synchronized(&self) -> bool {
        self.member.access_flags & ACC_SYNCHRONIZED != 0
    }
    pub fn is_bridge(&self) -> bool {
        self.member.access_flags & ACC_BRIDGE != 0
    }
    pub fn is_varargs(&self) -> bool {
        self.member.access_flags & ACC_VARARGS != 0
    }
    pub fn is_native(&self) -> bool {
        self.member.access_flags & ACC_NATIVE != 0
    }
    pub fn is_abstract(&self) -> bool {
        self.member.access_flags & ACC_ABSTRACT != 0
    }
    pub fn is_strict(&self) -> bool {
        self.member.access_flags & ACC_STRICT != 0
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

    pub fn name(&self) -> &str {
        self.member.name()
    }
    pub fn descriptor(&self) -> &str {
        self.member.descriptor()
    }
    pub fn class(&self) -> Arc<RwLock<Class>> {
        self.member.class()
    }
    pub fn is_accessible_to(&self, d: Arc<RwLock<Class>>) -> bool {
        self.member.is_accessible_to(d)
    }

    pub fn max_locals(&self) -> u16 {
        self.max_locals
    }
    pub fn max_stack(&self) -> u16 {
        self.max_stack
    }
    pub fn code(&self) -> Arc<Vec<u8>> {
        self.code.clone()
    }
    pub fn arg_slot_count(&self) -> usize {
        self.arg_slot_count
    }
    fn calc_arg_slot_count(member:&ClassMember) -> usize {
        let mut arg_slot_count = 0usize;
        let parsed_descriptor = MethodDescriptorParser::parse(member.descriptor());
        for param_type in &parsed_descriptor.parameter_types {
            arg_slot_count += 1;
            if param_type == "J" || param_type == "D" {
                arg_slot_count += 1;
            }
        }
        if member.is_static() {
            arg_slot_count += 1;
        }
        arg_slot_count
    }
}