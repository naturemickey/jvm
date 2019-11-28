pub struct ConstantPool {
    constant_infos: Vec<ConstantInfo>
}

impl ConstantPool {
    fn read_constant_pool(reader: &mut ClassReader) -> Arc<RwLock<ConstantPool>> {
        let cp_count = reader.read_u16();

        let cp = Arc::new(RwLock::new(ConstantPool { constant_infos: Vec::with_capacity(0) }));


        let mut constant_infos = Vec::with_capacity(cp_count as usize);

        //println!("debug log : cp_count is {}", cp_count);

        // 索引从1开始
        constant_infos.push(ConstantInfo::Empty);
        let mut i = 1u16;
        while i < cp_count {
            //println!("debug log : cp_index is {}", i);

            let constant_info = ConstantInfo::read_constant_info(reader, cp.clone());
            match &constant_info {
                ConstantInfo::Long(_) => {
                    constant_infos.push(ConstantInfo::Empty);
                    i += 1;
                    //println!("debug log : ConstantInfo::Long");
                }
                ConstantInfo::Double(_) => {
                    constant_infos.push(ConstantInfo::Empty);
                    i += 1;
                    //println!("debug log : ConstantInfo::Double");
                }
                _ => {}
            }
            constant_infos.push(constant_info);

            i += 1;
        }

        cp.write().unwrap().constant_infos = constant_infos;

        cp
    }

    pub fn get_constant_info(&self, index: u16) -> &ConstantInfo {
        match self.constant_infos.get(index as usize) {
            Some(c) => c,
            None => panic!("impossible.")
        }
    }

    fn name_and_type(&self, index: u16) -> (Arc<String>, Arc<String>) {
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

    fn class_name(&self, class_index: u16) -> Arc<String> {
        let constant_info = self.get_constant_info(class_index);

        match constant_info {
            ConstantInfo::Class(info) => info.name(),
            _ => panic!("impossible.")
        }
    }

    fn get_utf8(&self, index: u16) -> Arc<String> {
        let constant_info = self.get_constant_info(index);

        match constant_info {
            ConstantInfo::Utf8(info) => info.string(),
            _ => panic!("impossible.")
        }
    }

    pub fn constants_count(&self) -> u16 {
        self.constant_infos.len() as u16
    }
}
