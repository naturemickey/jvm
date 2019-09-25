pub struct MemberInfo<'a> {
    cp: &'a ConstantPool,
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

impl<'a> MemberInfo<'a> {
    fn read_member(reader: &'a mut ClassReader, cp: &'a ConstantPool) -> MemberInfo<'a> {
        let access_flags = reader.read_u16();
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        let attributes = AttributeInfo::read_attributes(reader, cp);

        Self { cp, access_flags, name_index, descriptor_index, attributes }
    }

    fn read_members(reader: &'a mut ClassReader, cp: &'a ConstantPool) -> Vec<MemberInfo<'a>> {
        let mut member_count = reader.read_u16();
        let mut members = Vec::new();
        while --member_count >= 0 {
            members.push(Self::read_member(reader, cp));
        }
        members
    }

    pub fn access_flgs(&self) -> u16 {
        self.access_flags
    }

    pub fn name(&self) -> &str {
        self.cp.get_utf8(self.name_index)
    }

    pub fn descriptor(&self) -> &str {
        self.cp.get_utf8(self.descriptor_index)
    }
}