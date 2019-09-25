pub struct ConstantPool {
    constant_info_s: Box<Vec<dyn ConstantInfo>>
}

impl ConstantPool {
    fn read_constant_pool(reader: &mut ClassReader) -> ConstantPool {
        let mut cp_count = reader.read_u16();
        let constant_info_s = Boxl::new(Vec::new());
        let mut cp = ConstantPool { constant_info_s };

        // todo 索引从1开始
        while --cp_count >= 0 {}

        cp
    }

    fn get_constant_info(&self, index: u16) -> &dyn ConstantInfo {
        &self.constant_info_s[index]
    }

    fn get_name_and_type(&self, index: u16) -> (&str, &str) {
        // todo
    }

    fn class_name(&self, class_index: u16) -> &str {}

    fn get_utf8(&self, index: u16) -> &str {}
}
