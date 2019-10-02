const CONSTANT_CLASS______________: u8 = 7;
const CONSTANT_FIELDREF___________: u8 = 9;
const CONSTANT_METHODREF__________: u8 = 10;
const CONSTANT_INTERFACE_METHODREF: u8 = 11;
const CONSTANT_STRING_____________: u8 = 8;
const CONSTANT_INTGER_____________: u8 = 3;
const CONSTANT_FLOAT______________: u8 = 4;
const CONSTANT_LONG_______________: u8 = 5;
const CONSTANT_DOUBLE_____________: u8 = 6;
const CONSTANT_NAME_AND_TYPE______: u8 = 12;
const CONSTANT_UTF8_______________: u8 = 1;
const CONSTANT_METHOD_HANDLE______: u8 = 15;
const CONSTANT_METHOD_TYPE________: u8 = 16;
const CONSTANT_INOVKE_DYNAMIC_____: u8 = 18;

fn read_constant_info(reader: &mut ClassReader) -> Box<dyn ConstantInfo> {
    let tag = reader.read_u8();
    new_constant_info(tag, reader)
}

fn new_constant_info(tag: u8, reader: &mut ClassReader) -> Box<dyn ConstantInfo> {
    match tag {
        CONSTANT_UTF8_______________ => Box::new(ConstantUtf8Info::new(reader)),
        CONSTANT_INTGER_____________ => Box::new(ConstantIntegerInfo::new(reader)),
        CONSTANT_FLOAT______________ => Box::new(ConstantFloatInfo::new(reader)),
        CONSTANT_LONG_______________ => Box::new(ConstantLongInfo::new(reader)),
        CONSTANT_DOUBLE_____________ => Box::new(ConstantDoubleInfo::new(reader)),
        CONSTANT_CLASS______________ => Box::new(ConstantClassInfo::new(reader)),
        CONSTANT_STRING_____________ => Box::new(ConstantStringInfo::new(reader)),
        CONSTANT_FIELDREF___________ => Box::new(ConstantFieldrefInfo::new(reader)),
        CONSTANT_METHODREF__________ => Box::new(ConstantMethodrefInfo::new(reader)),
        CONSTANT_INTERFACE_METHODREF => Box::new(ConstantInterfaceMethodrefInfo::new(reader)),
        CONSTANT_NAME_AND_TYPE______ => Box::new(ConstantNameAndTypeInfo::new(reader)),
        CONSTANT_METHOD_HANDLE______ => Box::new(ConstantMethodHandleInfo::new(reader)),
        CONSTANT_METHOD_TYPE________ => Box::new(ConstantMethodTypeInfo::new(reader)),
        CONSTANT_INOVKE_DYNAMIC_____ => Box::new(ConstantInvokeDynamicInfo::new(reader)),
        _ => panic!("java. lang. ClassFormatError: constant pool tag!"),
    }
}

trait ConstantInfo {}

struct ConstantEmptyInfo {}

impl ConstantInfo for ConstantEmptyInfo {}

const EMPTY_CONSTANT_INFO: ConstantEmptyInfo = ConstantEmptyInfo {};

struct ConstantIntegerInfo {
    integer32: i32
}

impl ConstantIntegerInfo {
    fn new(reader: &mut ClassReader) -> ConstantIntegerInfo {
        Self { integer32: reader.read_u32() as i32 }
    }
}

impl ConstantInfo for ConstantIntegerInfo {}

struct ConstantFloatInfo {
    float32: f32
}

impl ConstantFloatInfo {
    fn new(reader: &mut ClassReader) -> ConstantFloatInfo {
        Self { float32: f32::from_bits(reader.read_u32()) }
    }
}

impl ConstantInfo for ConstantFloatInfo {}

struct ConstantUtf8Info {
    string: String
}

impl ConstantUtf8Info {
    fn new(reader: &mut ClassReader) -> ConstantUtf8Info {
        let length = reader.read_u16();
        let bytes = reader.read_bytes(length as usize);
        let string = String::from_utf8(bytes).unwrap();
        Self { string }
    }
}

impl ConstantInfo for ConstantUtf8Info {}

struct ConstantLongInfo {
    integer64: i64
}

impl ConstantLongInfo {
    fn new(reader: &mut ClassReader) -> ConstantLongInfo {
        Self { integer64: reader.read_u64() as i64 }
    }
}

impl ConstantInfo for ConstantLongInfo {}

struct ConstantDoubleInfo {
    float64: f64
}

impl ConstantDoubleInfo {
    fn new(reader: &mut ClassReader) -> ConstantDoubleInfo {
        Self { float64: f64::from_bits(reader.read_u64()) }
    }
}

impl ConstantInfo for ConstantDoubleInfo {}

struct ConstantClassInfo {
    string_info: ConstantStringInfo
}

impl ConstantClassInfo {
    fn new(reader: &mut ClassReader) -> ConstantClassInfo {
        Self { string_info: ConstantStringInfo::new(reader) }
    }
    fn name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        self.string_info.string(cp)
    }
}

impl<'a> ConstantInfo for ConstantClassInfo {}

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

impl ConstantInfo for ConstantStringInfo {}

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
    member: ConstantMemberrefInfo
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

impl ConstantInfo for ConstantFieldrefInfo {}

struct ConstantMethodrefInfo {
    member: ConstantMemberrefInfo
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

impl ConstantInfo for ConstantMethodrefInfo {}

struct ConstantInterfaceMethodrefInfo {
    member: ConstantMemberrefInfo
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

impl ConstantInfo for ConstantInterfaceMethodrefInfo {}

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

impl ConstantInfo for ConstantNameAndTypeInfo {}

struct ConstantMethodHandleInfo {}

impl ConstantMethodHandleInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodHandleInfo {
        unimplemented!()
    }
}

impl ConstantInfo for ConstantMethodHandleInfo {}

struct ConstantMethodTypeInfo {}

impl ConstantMethodTypeInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodTypeInfo {
        unimplemented!()
    }
}

impl ConstantInfo for ConstantMethodTypeInfo {}

struct ConstantInvokeDynamicInfo {}

impl ConstantInvokeDynamicInfo {
    fn new(reader: &mut ClassReader) -> ConstantInvokeDynamicInfo {
        unimplemented!()
    }
}

impl ConstantInfo for ConstantInvokeDynamicInfo {}