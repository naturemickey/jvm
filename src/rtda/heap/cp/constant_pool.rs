struct ConstantPool<'a> {
    class: &'a Class<'a>,
    consts: Vec<Constant>,
}

impl<'a> ConstantPool<'a> {
    fn new(class: &'a Class, cf_cp: &'a classfile::ConstantPool) -> ConstantPool<'a> {
        unimplemented!()
    }
}