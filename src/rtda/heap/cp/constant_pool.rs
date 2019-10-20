pub struct ConstantPool<'a> {
     class: &'a Class<'a>,
    consts: Vec<Constant>,
}

impl<'a> ConstantPool <'a> {
    fn new(cf_cp: Arc<classfile::ConstantPool>, class:&'a Class<'a>) -> ConstantPool {
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
                ConstantInfo::Class(info) => {unimplemented!()}
                ConstantInfo::FieldRef(info) => {unimplemented!()}
                ConstantInfo::MethodRef(info) => {unimplemented!()}
                ConstantInfo::InterfaceMethodRef(info) => {unimplemented!()}
                ConstantInfo::Utf8(info) => {unimplemented!()}
                ConstantInfo::NameAndType(info) => {unimplemented!()}
                ConstantInfo::MethodHandle(info) => {unimplemented!()}
                ConstantInfo::MethodType(info) => {unimplemented!()}
                ConstantInfo::Dynamic(info) => {unimplemented!()}
                ConstantInfo::InvokeDynamic(info) => {unimplemented!()}
                ConstantInfo::Module(info) => {unimplemented!()}
                ConstantInfo::Package(info) => {unimplemented!()}
                ConstantInfo::Empty => {unimplemented!()}
            };
            i += more;
        }
        Self{class, consts}
    }
}