pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Box<Vec<u16>>,
    fields: Box<Vec<Box<MemberInfo>>>,
    methods: Box<Vec<Box<MemberInfo>>>,
    attributes: Box<Vec<AttributeInfo>>,
}

impl ClassFile {
    pub fn parse(class_data: Vec<u8>) -> ClassFile {
        
    }
}