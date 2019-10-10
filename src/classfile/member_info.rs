pub struct MemberInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

impl MemberInfo {
    fn read_member(reader: &mut ClassReader, cp: &ConstantPool) -> MemberInfo {
        let access_flags = reader.read_u16();
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        let attributes = AttributeInfo::read_attributes(reader, cp);

        Self { access_flags, name_index, descriptor_index, attributes }
    }

    fn read_members(reader: &mut ClassReader, cp: &ConstantPool) -> Vec<MemberInfo> {
        let member_count = reader.read_u16();
        let mut members = Vec::new();
        for _ in 0..member_count {
            members.push(Self::read_member(reader, cp));
        }
        members
    }

    pub fn access_flgs(&self) -> u16 {
        self.access_flags
    }

    pub fn name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        cp.get_utf8(self.name_index)
    }

    pub fn descriptor<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        cp.get_utf8(self.descriptor_index)
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
}