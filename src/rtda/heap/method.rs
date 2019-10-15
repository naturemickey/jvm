pub struct Method<'a> {
    member: ClassMember<'a>,
    max_stack: u16,
    max_locals: u16,
    code: Arc<Vec<u8>>,
}

impl<'a> Method<'a> {
    pub fn new_methods(class: Arc<Class<'a>>, cf_methods: &'a Vec<MemberInfo>) -> Vec<Method<'a>> {
        let mut methods = Vec::with_capacity(cf_methods.len());
        for cf_method in cf_methods {
            let member = ClassMember::new(class.clone(), cf_method);
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

    pub fn is_static(&self) -> bool {
        unimplemented!()
    }

    pub fn name(&self) -> &str {
        unimplemented!()
    }

    pub fn descriptor(&self) -> &str {
        unimplemented!()
    }
}