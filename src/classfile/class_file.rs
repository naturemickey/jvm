pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: Arc<ConstantPool>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<MemberInfo>,
    methods: Vec<MemberInfo>,
    attributes: Vec<AttributeInfo>,
}


impl ClassFile {
    pub fn parse(class_data: Vec<u8>) -> ClassFile {
        let mut cr = ClassReader::new(class_data);
        let cf = ClassFile::read(&mut cr);
        cf
    }

    fn read(reader: &mut ClassReader) -> ClassFile {
        let magic = ClassFile::read_and_check_magic(reader);
        let (minor_version, major_version) = ClassFile::read_and_check_version(reader);
        let cp = Arc::new(ConstantPool::read_constant_pool(reader));
        let access_flags = reader.read_u16();
        let this_class = reader.read_u16();
        let super_class = reader.read_u16();
        let interfaces = reader.read_u16s();
        let fields = MemberInfo::read_members(reader, cp.clone());
        let methods = MemberInfo::read_members(reader, cp.clone());
        let attributes = AttributeInfo::read_attributes(reader, cp.clone());

        Self { magic, minor_version, major_version, constant_pool: cp.clone(), access_flags, this_class, super_class, interfaces, fields, methods, attributes }
    }

    fn read_and_check_magic(reader: &mut ClassReader) -> u32 {
        let magic: u32 = reader.read_u32();
        if magic != 0xCAFEBABE {
            panic!("java.lang.ClassFormatError: magic!");
        }
        magic
    }

    fn read_and_check_version(reader: &mut ClassReader) -> (u16, u16) {
        let minor_version: u16 = reader.read_u16();
        let major_version: u16 = reader.read_u16();

        let res = (minor_version, major_version);

        match res {
            (_, 45) => res,
            (0, 46) => res,
            (0, 47) => res,
            (0, 48) => res,
            (0, 49) => res,
            (0, 50) => res,
            (0, 51) => res,
            (0, 52) => res,
            _ => panic!("java. lang. UnsupportedClassVersionError!")
        }
    }

    // getter
    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }
    pub fn major_version(&self) -> u16 {
        self.major_version
    }
    pub fn constant_pool(&self) -> Arc<ConstantPool> {
        self.constant_pool.clone()
    }
    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }
    pub fn fields(&self) -> &Vec<MemberInfo> {
        &self.fields
    }
    pub fn methods(&self) -> &Vec<MemberInfo> {
        &self.methods
    }
    pub fn class_name(&self) -> &str {
        self.constant_pool.class_name(self.this_class)
    }
    pub fn super_class_name(&self) -> &str {
        if self.super_class > 0 {
            self.constant_pool.class_name(self.super_class)
        } else {
            ""
        }
    }
    pub fn interface_names(&self) -> Vec<&str> {
        let mut names = Vec::new();
        for i in &self.interfaces {
            names.push(self.constant_pool.class_name(*i));
        }
        names
    }
}