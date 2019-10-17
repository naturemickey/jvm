pub struct ConstantPool {
    // class: &'a Class<'a>,
    consts: Vec<Constant>,
}

impl ConstantPool {
    fn new(cf_cp: Arc<classfile::ConstantPool>) -> ConstantPool {
        unimplemented!()
    }
}