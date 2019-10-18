pub struct ConstantPool {
    // class: &'a Class<'a>,
    consts: Vec<Constant>,
}

impl ConstantPool {
    fn new(cf_cp: Arc<classfile::ConstantPool>) -> ConstantPool {
        let cp_count = cf_cp.constants_count();
        let mut consts = Vec::with_capacity(cp_count as usize);
        let mut i = 1;
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
                ConstantInfo::String(info) => Constant::String(info.string(cf_cp.borrow()).to_string()),
                ConstantInfo::Class(info) => {}
                ConstantInfo::FieldRef(info) => {}
                ConstantInfo::MethodRef(info) => {}
                ConstantInfo::InterfaceMethodRef(info) => {}
                ConstantInfo::Utf8(info) => {}
                ConstantInfo::NameAndType(info) => {}
                ConstantInfo::MethodHandle(info) => {}
                ConstantInfo::MethodType(info) => {}
                ConstantInfo::Dynamic(info) => {}
                ConstantInfo::InvokeDynamic(info) => {}
                ConstantInfo::Module(info) => {}
                ConstantInfo::Package(info) => {}
                ConstantInfo::Empty => {}
            };
            i += more;
        }
        consts
    }
}