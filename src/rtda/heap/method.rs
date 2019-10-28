pub struct Method {
    member: ClassMember,
    max_stack: u16,
    max_locals: u16,
    code: Arc<Vec<u8>>,
}

impl Method {
    pub fn new_methods(arc_class: Arc<Class>, cf_methods: &Vec<MemberInfo>) -> Vec<Method> {
        let mut methods = Vec::with_capacity(cf_methods.len());
        for cf_method in cf_methods {
            let member = ClassMember::new(arc_class.clone(), cf_method);
            let (max_stack, max_locals, code) = Self::copy_attributes(cf_method);
            methods.push(Self { member, max_stack, max_locals, code });
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
    fn name(&self) -> &str {
        self.member.name()
    }
    fn descriptor(&self) -> &str {
        self.member.descriptor()
    }
    fn class(&self) -> &Class {
        self.member.class()
    }
    fn is_accessible_to(&self, d: &Class) -> bool {
        self.member.is_accessible_to(d)
    }
}