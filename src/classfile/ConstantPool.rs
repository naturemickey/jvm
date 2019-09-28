pub struct ConstantPool {
    constant_info_s: Vec<Box<dyn ConstantInfo>>
}

impl ConstantPool {
    fn read_constant_pool(reader: &mut ClassReader) -> ConstantPool {
        let mut cp_count = reader.read_u16();
        let mut constant_info_s = Vec::new();
        let mut cp = ConstantPool { constant_info_s };

        // 索引从1开始
        constant_info_s.push(EMPTY_CONSTANT_INFO);
        for i in 1..cp_count {
            // todo
        }

        cp
    }

    fn get_constant_info(&self, index: u16) -> &dyn ConstantInfo {
        self.constant_info_s.get(index as usize).unwrap().as_ref()
    }

    fn get_name_and_type(&self, index: u16) -> (&str, &str) {
        // todo
    }

    fn class_name(&self, class_index: u16) -> &str {}

    fn get_utf8(&self, index: u16) -> &str {}
}
