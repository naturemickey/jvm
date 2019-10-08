pub struct ConstantPool {
    constant_info_s: Vec<ConstantInfo>
}

impl ConstantPool {
    fn read_constant_pool(reader: &mut ClassReader) -> ConstantPool {
        let cp_count = reader.read_u16();
        let mut constant_info_s: Vec<ConstantInfo> = Vec::new();

        //println!("debug log : cp_count is {}", cp_count);

        // 索引从1开始
        constant_info_s.push(ConstantInfo::Empty);
        let mut i = 1u16;
        while i < cp_count {
            //println!("debug log : cp_index is {}", i);

            let constant_info = ConstantInfo::read_constant_info(reader);
            match &constant_info {
                ConstantInfo::Long(_) => {
                    constant_info_s.push(ConstantInfo::Empty);
                    i += 1;
                    //println!("debug log : ConstantInfo::Long");
                }
                ConstantInfo::Double(_) => {
                    constant_info_s.push(ConstantInfo::Empty);
                    i += 1;
                    //println!("debug log : ConstantInfo::Double");
                }
                _ => {}
            }
            constant_info_s.push(constant_info);

            i += 1;
        }

        ConstantPool { constant_info_s }
    }

    fn get_constant_info(&self, index: u16) -> &ConstantInfo {
        self.constant_info_s.get(index as usize).unwrap()
    }

    fn name_and_type(&self, index: u16) -> (&str, &str) {
        let constant_info = self.get_constant_info(index);

        match constant_info {
            ConstantInfo::NameAndType(info) => {
                let _name = self.get_utf8(info.name_index);
                let _type = self.get_utf8(info.descriptor_index);
                (_name, _type)
            }
            _ => panic!("impossible.")
        }
    }

    fn class_name(&self, class_index: u16) -> &str {
        let constant_info = self.get_constant_info(class_index);

        match constant_info {
            ConstantInfo::Class(info) => info.name(self),
            _ => panic!("impossible.")
        }
    }

    fn get_utf8(&self, index: u16) -> &str {
        let constant_info = self.get_constant_info(index);

        match constant_info {
            ConstantInfo::Utf8(info) => &info.string,
            _ => panic!("impossible.")
        }
    }

    pub fn constants_count(&self) -> usize {
        self.constant_info_s.len()
    }
}
