pub struct ConstantPool {
    class: Option<Arc<Class>>,
    consts: Vec<Constant>,
}

impl ConstantPool {
    fn new(cf_cp: Arc<classfile::ConstantPool>, class: Option<Arc<Class>>) -> Arc<ConstantPool> {
        let cp_count = cf_cp.constants_count();
        let mut i = 1;
        let mut rt_cp = Self { class, consts: Vec::with_capacity(cp_count as usize) };

        while i < cp_count {
            let mut more = 1;
            let const_ = match cf_cp.get_constant_info(i) {
                ConstantInfo::Integer(info) => Constant::Integer(info.value()),
                ConstantInfo::Float(info) => Constant::Float(info.value()),
                ConstantInfo::Long(info) => {
                    more = 2;
                    Constant::Long(info.value())
                }
                ConstantInfo::Double(info) => {
                    more = 2;
                    Constant::Double(info.value())
                }
                ConstantInfo::String(info) => Constant::String(info.string().to_string()),
                ConstantInfo::Class(info) => Constant::Class(ClassRef::new(info, &rt_cp)),
                ConstantInfo::FieldRef(info) => Constant::FieldRef(FieldRef::new(info, &rt_cp)),
                ConstantInfo::MethodRef(info) => Constant::MethodRef(MethodRef::new(info, &rt_cp)),
                ConstantInfo::InterfaceMethodRef(info) => Constant::InterfaceMethodRef(InterfaceMethodRef::new(info, &rt_cp)),
                ConstantInfo::Utf8(info) => { unimplemented!() }
                ConstantInfo::NameAndType(info) => { unimplemented!() }
                ConstantInfo::MethodHandle(info) => { unimplemented!() }
                ConstantInfo::MethodType(info) => { unimplemented!() }
                ConstantInfo::Dynamic(info) => { unimplemented!() }
                ConstantInfo::InvokeDynamic(info) => { unimplemented!() }
                ConstantInfo::Module(info) => { unimplemented!() }
                ConstantInfo::Package(info) => { unimplemented!() }
                ConstantInfo::Empty => { unimplemented!() }
            };
            rt_cp.consts.push(const_);
            i += more;
        }
        Arc::new(rt_cp)
    }

    fn set_class(&mut self, class: Option<Arc<Class>>) {
        self.class = class;
    }

    fn class(&self) -> Arc<Class> {
        match &self.class {
            Some(c) => c.clone(),
            None => panic!("impossible.")
        }
    }
}