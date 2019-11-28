pub struct MemberInfo {
    cp: Arc<RwLock<ConstantPool>>,
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

impl MemberInfo {
    fn read_member(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> MemberInfo {
        let access_flags = reader.read_u16();
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        let attributes = AttributeInfo::read_attributes(reader, cp.clone());

        Self { cp:cp.clone(), access_flags, name_index, descriptor_index, attributes }
    }

    fn read_members(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> Vec<MemberInfo> {
        let member_count = reader.read_u16();
        let mut members = Vec::new();
        for _ in 0..member_count {
            members.push(Self::read_member(reader, cp.clone()));
        }
        members
    }

    pub fn access_flgs(&self) -> u16 {
        self.access_flags
    }

    pub fn name(&self) -> Arc<String> {
        self.cp.read().unwrap().get_utf8(self.name_index)
    }

    pub fn descriptor(&self) -> Arc<String> {
        self.cp.read().unwrap().get_utf8(self.descriptor_index)
    }

    pub fn code_attribute(&self) -> Option<&CodeAttribute> {
        for attr in &self.attributes {
            match attr {
                AttributeInfo::Code(ca) => return Some(ca),
                _ => {}
            };
        }
        None
    }

    pub fn constant_value_attribute(&self) -> Option<&ConstantValueAttribute> {
        for attr in &self.attributes {
            match attr {
                AttributeInfo::ConstantValue(cva) => return Some(cva),
                _ => {}
            };
        }
        None
    }
}

impl Debug for MemberInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}{}", self.name(), self.descriptor())
    }
}