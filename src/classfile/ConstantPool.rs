pub struct ConstantPool {
    constant_info_s: Vec<Box<dyn ConstantInfo>>
}

impl ConstantPool {
    fn read_constant_pool(reader: &mut ClassReader) -> ConstantPool {
        let cp_count = reader.read_u16();
        let mut constant_info_s: Vec<Box<dyn ConstantInfo>> = Vec::new();

        // 索引从1开始
        constant_info_s.push(Box::new(EMPTY_CONSTANT_INFO));
        for _ in 1..cp_count {
            constant_info_s.push(read_constant_info(reader));
        }

        ConstantPool { constant_info_s }
    }

    fn get_constant_info(&self, index: u16) -> &dyn ConstantInfo {
        self.constant_info_s.get(index as usize).unwrap().as_ref()
    }

    fn name_and_type(&self, index: u16) -> (&str, &str) {
        // todo
        unimplemented!()
    }

    fn class_name(&self, class_index: u16) -> &str {
        unimplemented!()
    }

    fn get_utf8(&self, index: u16) -> &str {
        unimplemented!()
    }
}
