const CONSTANT_CLASS_______________: u8 = 7;
const CONSTANT_FIELD_REF___________: u8 = 9;
const CONSTANT_METHOD_REF__________: u8 = 10;
const CONSTANT_INTERFACE_METHOD_REF: u8 = 11;
const CONSTANT_STRING______________: u8 = 8;
const CONSTANT_INTEGER_____________: u8 = 3;
const CONSTANT_FLOAT_______________: u8 = 4;
const CONSTANT_LONG________________: u8 = 5;
const CONSTANT_DOUBLE______________: u8 = 6;
const CONSTANT_NAME_AND_TYPE_______: u8 = 12;
const CONSTANT_UTF8________________: u8 = 1;
const CONSTANT_METHOD_HANDLE_______: u8 = 15;
const CONSTANT_METHOD_TYPE_________: u8 = 16;
const CONSTANT_INVOKE_DYNAMIC______: u8 = 18;

fn read_constant_info(reader: &mut ClassReader) -> ConstantInfo {
    let tag = reader.read_u8();
    new_constant_info(tag, reader)
}

fn new_constant_info(tag: u8, reader: &mut ClassReader) -> ConstantInfo {
    match tag {
        CONSTANT_UTF8________________ => ConstantInfo::Utf8(ConstantUtf8Info::new(reader)),
        CONSTANT_INTEGER_____________ => ConstantInfo::Integer(ConstantIntegerInfo::new(reader)),
        CONSTANT_FLOAT_______________ => ConstantInfo::Float(ConstantFloatInfo::new(reader)),
        CONSTANT_LONG________________ => ConstantInfo::Long(ConstantLongInfo::new(reader)),
        CONSTANT_DOUBLE______________ => ConstantInfo::Double(ConstantDoubleInfo::new(reader)),
        CONSTANT_CLASS_______________ => ConstantInfo::Class(ConstantClassInfo::new(reader)),
        CONSTANT_STRING______________ => ConstantInfo::String(ConstantStringInfo::new(reader)),
        CONSTANT_FIELD_REF___________ => ConstantInfo::FieldRef(ConstantFieldrefInfo::new(reader)),
        CONSTANT_METHOD_REF__________ => ConstantInfo::MethodRef(ConstantMethodrefInfo::new(reader)),
        CONSTANT_INTERFACE_METHOD_REF => ConstantInfo::InterfaceMethodRef(ConstantInterfaceMethodrefInfo::new(reader)),
        CONSTANT_NAME_AND_TYPE_______ => ConstantInfo::NameAndType(ConstantNameAndTypeInfo::new(reader)),
        CONSTANT_METHOD_HANDLE_______ => ConstantInfo::MethodHandle(ConstantMethodHandleInfo::new(reader)),
        CONSTANT_METHOD_TYPE_________ => ConstantInfo::MethodType(ConstantMethodTypeInfo::new(reader)),
        CONSTANT_INVOKE_DYNAMIC______ => ConstantInfo::InvokeDynamic(ConstantInvokeDynamicInfo::new(reader)),
        _ => panic!("java. lang. ClassFormatError: constant pool tag!"),
    }
}

enum ConstantInfo {
    Empty,
    Utf8(ConstantUtf8Info),
    Integer(ConstantIntegerInfo),
    Float(ConstantFloatInfo),
    Long(ConstantLongInfo),
    Double(ConstantDoubleInfo),
    Class(ConstantClassInfo),
    String(ConstantStringInfo),
    FieldRef(ConstantFieldrefInfo),
    MethodRef(ConstantMethodrefInfo),
    InterfaceMethodRef(ConstantInterfaceMethodrefInfo),
    NameAndType(ConstantNameAndTypeInfo),
    MethodHandle(ConstantMethodHandleInfo),
    MethodType(ConstantMethodTypeInfo),
    InvokeDynamic(ConstantInvokeDynamicInfo),
}


struct ConstantIntegerInfo {
    integer32: i32,
}

impl ConstantIntegerInfo {
    fn new(reader: &mut ClassReader) -> ConstantIntegerInfo {
        Self { integer32: reader.read_u32() as i32 }
    }
}

struct ConstantFloatInfo {
    float32: f32,
}

impl ConstantFloatInfo {
    fn new(reader: &mut ClassReader) -> ConstantFloatInfo {
        Self { float32: f32::from_bits(reader.read_u32()) }
    }
}

struct ConstantUtf8Info {
    string: String,
}

impl ConstantUtf8Info {
    fn new(reader: &mut ClassReader) -> ConstantUtf8Info {
        let length = reader.read_u16();
        let bytes = reader.read_bytes(length as usize);
        let string = String::from_utf8(bytes).unwrap();
        Self { string }
    }
}

struct ConstantLongInfo {
    integer64: i64,
}

impl ConstantLongInfo {
    fn new(reader: &mut ClassReader) -> ConstantLongInfo {
        Self { integer64: reader.read_u64() as i64 }
    }
}

struct ConstantDoubleInfo {
    float64: f64,
}

impl ConstantDoubleInfo {
    fn new(reader: &mut ClassReader) -> ConstantDoubleInfo {
        Self { float64: f64::from_bits(reader.read_u64()) }
    }
}

struct ConstantClassInfo {
    string_info: ConstantStringInfo,
}

impl ConstantClassInfo {
    fn new(reader: &mut ClassReader) -> ConstantClassInfo {
        Self { string_info: ConstantStringInfo::new(reader) }
    }
    fn name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        self.string_info.string(cp)
    }
}

struct ConstantStringInfo {
    string_index: u16,
}

impl ConstantStringInfo {
    fn new(reader: &mut ClassReader) -> ConstantStringInfo {
        let string_index = reader.read_u16();
        Self { string_index }
    }
    fn string<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        cp.get_utf8(self.string_index)
    }
}

struct ConstantMemberrefInfo {
    class_index: u16,
    name_and_type_index: u16,
}

impl ConstantMemberrefInfo {
    fn new(reader: &mut ClassReader) -> ConstantMemberrefInfo {
        let class_index = reader.read_u16();
        let name_and_type_index = reader.read_u16();
        Self { class_index, name_and_type_index }
    }
    fn class_name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        cp.class_name(self.class_index)
    }
    fn name_and_descriptor<'a>(&'a self, cp: &'a ConstantPool) -> (&'a str, &'a str) {
        cp.name_and_type(self.name_and_type_index)
    }
}

struct ConstantFieldrefInfo {
    member: ConstantMemberrefInfo,
}

impl ConstantFieldrefInfo {
    fn new(reader: &mut ClassReader) -> ConstantFieldrefInfo {
        Self { member: ConstantMemberrefInfo::new(reader) }
    }
    fn class_name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        self.member.class_name(cp)
    }
    fn name_and_descriptor<'a>(&'a self, cp: &'a ConstantPool) -> (&'a str, &'a str) {
        self.member.name_and_descriptor(cp)
    }
}

struct ConstantMethodrefInfo {
    member: ConstantMemberrefInfo,
}

impl ConstantMethodrefInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodrefInfo {
        Self { member: ConstantMemberrefInfo::new(reader) }
    }
    fn class_name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        self.member.class_name(cp)
    }
    fn name_and_descriptor<'a>(&'a self, cp: &'a ConstantPool) -> (&'a str, &'a str) {
        self.member.name_and_descriptor(cp)
    }
}

struct ConstantInterfaceMethodrefInfo {
    member: ConstantMemberrefInfo,
}

impl ConstantInterfaceMethodrefInfo {
    fn new(reader: &mut ClassReader) -> ConstantInterfaceMethodrefInfo {
        Self { member: ConstantMemberrefInfo::new(reader) }
    }
    fn class_name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        self.member.class_name(cp)
    }
    fn name_and_descriptor<'a>(&'a self, cp: &'a ConstantPool) -> (&'a str, &'a str) {
        self.member.name_and_descriptor(cp)
    }
}

struct ConstantNameAndTypeInfo {
    name_index: u16,
    descriptor_index: u16,
}

impl ConstantNameAndTypeInfo {
    fn new(reader: &mut ClassReader) -> ConstantNameAndTypeInfo {
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        Self { name_index, descriptor_index }
    }
}

struct ConstantMethodHandleInfo {
    reference_kind: u8,
    reference_index: u16,
}

impl ConstantMethodHandleInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodHandleInfo {
        let reference_kind = reader.read_u8();
        let reference_index = reader.read_u16();
        Self { reference_kind, reference_index }
    }
}

struct ConstantMethodTypeInfo {
    descriptor_index: u16
}

impl ConstantMethodTypeInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodTypeInfo {
        Self { descriptor_index: reader.read_u16() }
    }
}

struct ConstantInvokeDynamicInfo {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
}

impl ConstantInvokeDynamicInfo {
    fn new(reader: &mut ClassReader) -> ConstantInvokeDynamicInfo {
        let bootstrap_method_attr_index = reader.read_u16();
        let name_and_type_index = reader.read_u16();
        Self { bootstrap_method_attr_index, name_and_type_index }
    }
}
