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
        let mut cr = ClassReader::new(class_data);
        let mut cf = ClassFile::new(&mut cr);
        cf
    }

    fn new(reader: &mut ClassReader) -> ClassFile {}
    fn read_and_check_magic(&self, reader: &mut ClassReader) {}
    fn read_and_check_version(&self, reader: &mut ClassReader) {}

    // getter
    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }
    pub fn major_version(&self) -> u16 {
        self.major_version
    }
    pub fn constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }
    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }
    pub fn fields(&self) -> Box<Vec<Box<MemberInfo>>> {
        self.fields
    }
    pub fn methods(&self) -> Box<Vec<Box<MemberInfo>>> {
        self.methods
    }
    pub fn class_name(&self) -> String {}
    pub fn super_class_name(&self) -> String {}
    pub fn interface_names(&self) -> Vec<String> {}
}