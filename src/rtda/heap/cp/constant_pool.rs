pub struct ConstantPool {
    class: Option<Arc<RwLock<Class>>>,
    consts: Vec<Constant>,
}

impl ConstantPool {
    fn new(cf_cp: Arc<RwLock<classfile::ConstantPool>>, class: Option<Arc<RwLock<Class>>>) -> Arc<RwLock<ConstantPool>> {
        let cp_count = cf_cp.read().unwrap().constants_count();
        let mut i = 1;
        let rt_cp = Arc::new(RwLock::new(Self { class, consts: Vec::with_capacity(cp_count as usize) }));

        while i < cp_count {
            let mut more = 1;
            let const_ = match cf_cp.read().unwrap().get_constant_info(i) {
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
                ConstantInfo::Class(info) => Constant::Class(ClassRef::new(info, rt_cp.clone())),
                ConstantInfo::FieldRef(info) => Constant::FieldRef(FieldRef::new(info, rt_cp.clone())),
                ConstantInfo::MethodRef(info) => Constant::MethodRef(MethodRef::new(info, rt_cp.clone())),
                ConstantInfo::InterfaceMethodRef(info) => Constant::InterfaceMethodRef(InterfaceMethodRef::new(info, rt_cp.clone())),
                ConstantInfo::Utf8(info) => Constant::Utf8(info.string().to_string()),
                ConstantInfo::NameAndType(info) => { Constant::Empty /*todo*/ }
                ConstantInfo::MethodHandle(info) => { Constant::Empty /*todo*/ }
                ConstantInfo::MethodType(info) => { Constant::Empty /*todo*/ }
                ConstantInfo::Dynamic(info) => { Constant::Empty /*todo*/ }
                ConstantInfo::InvokeDynamic(info) => { Constant::Empty /*todo*/ }
                ConstantInfo::Module(info) => { Constant::Empty /*todo*/ }
                ConstantInfo::Package(info) => { Constant::Empty /*todo*/ }
                ConstantInfo::Empty => { Constant::Empty /*todo*/ }
            };
            // need confirm there is only one pointer to pc.
            rt_cp.write().unwrap().consts.push(const_);
            i += more;
        }
        rt_cp
    }

    fn set_class(&mut self, class: Option<Arc<RwLock<Class>>>) {
        self.class = class;
    }

    fn class(&self) -> Arc<RwLock<Class>> {
        match &self.class {
            Some(c) => c.clone(),
            None => panic!("impossible.")
        }
    }

    pub fn get_constant(&self, cp_index: u16) -> &Constant {
        &self.consts[cp_index as usize]
    }

    pub fn get_constant_mut(&mut self, cp_index: u16) -> &mut Constant {
        &mut self.consts[cp_index as usize]
    }
}