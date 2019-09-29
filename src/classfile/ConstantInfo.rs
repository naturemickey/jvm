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

fn read_constant_info(reader: &mut ClassReader, cp: &ConstantPool) -> Box<dyn ConstantInfo> {
    let tag = reader.read_u8();
    new_constant_info(tag, reader, cp)
}

fn new_constant_info(tag: u8, reader: &mut ClassReader, cp: &ConstantPool) -> Box<dyn ConstantInfo> {
    let mut c: Box<dyn ConstantInfo> = match tag {
        CONSTANT_UTF8_______________ => Box::new(ConstantUtf8Info::new(reader)),
        CONSTANT_INTGER_____________ => Box::new(ConstantIntegerInfo::new(reader)),
        CONSTANT_FLOAT______________ => Box::new(ConstantFloatInfo::new(reader)),
        CONSTANT_LONG_______________ => Box::new(ConstantLongInfo::new(reader)),
        CONSTANT_DOUBLE_____________ => Box::new(ConstantDoubleInfo::new(reader)),
        CONSTANT_CLASS______________ => Box::new(ConstantClassInfo::new(reader, cp)),
        CONSTANT_STRING_____________ => Box::new(ConstantStringInfo::new(reader, cp)),
        CONSTANT_FIELDREF___________ => Box::new(ConstantFieldrefInfo::new(reader, ConstantMemberrefInfo::new(reader, cp))),
        CONSTANT_METHODREF__________ => Box::new(ConstantMethodrefInfo::new(reader, ConstantMemberrefInfo::new(reader, cp))),
        CONSTANT_INTERFACE_METHODREF => Box::new(ConstantInterfaceMethodrefInfo::new(reader, ConstantMemberrefInfo::new(reader, cp))),
        CONSTANT_NAME_AND_TYPE______ => Box::new(ConstantNameAndTypeInfo::new(reader)),
        CONSTANT_METHOD_HANDLE______ => Box::new(ConstantMethodHandleInfo::new(reader)),
        CONSTANT_METHOD_TYPE________ => Box::new(ConstantMethodTypeInfo::new(reader)),
        CONSTANT_INOVKE_DYNAMIC_____ => Box::new(ConstantInvokeDynamicInfo::new(reader)),
        _ => panic!("java. lang. ClassFormatError: constant pool tag!"),
    };
    c
}

trait ConstantInfo {}

struct ConstantEmptyInfo {}

impl ConstantInfo for ConstantEmptyInfo {}

const EMPTY_CONSTANT_INFO: Box<ConstantEmptyInfo> = Box::new(ConstantEmptyInfo {});

struct ConstantIntegerInfo {
    integer32: i32
}

impl ConstantIntegerInfo {
    fn new(reader: &mut ClassReader) -> ConstantIntegerInfo {
        ConstantIntegerInfo { integer32: reader.read_u32() as i32 }
    }
}

impl ConstantInfo for ConstantIntegerInfo {}

struct ConstantFloatInfo {
    float32: f32
}

impl ConstantFloatInfo {
    fn new(reader: &mut ClassReader) -> ConstantFloatInfo {
        ConstantFloatInfo { float32: reader.read_u32() as f32 }
    }
}

impl ConstantInfo for ConstantFloatInfo {}

struct ConstantUtf8Info {}

impl ConstantUtf8Info {
    fn new(reader: &mut ClassReader) -> ConstantUtf8Info {}
}

impl ConstantInfo for ConstantUtf8Info {}

struct ConstantLongInfo {}

impl ConstantLongInfo {
    fn new(reader: &mut ClassReader) -> ConstantLongInfo {}
}

impl ConstantInfo for ConstantLongInfo {}

struct ConstantDoubleInfo {}

impl ConstantDoubleInfo {
    fn new(reader: &mut ClassReader) -> ConstantDoubleInfo {}
}

impl ConstantInfo for ConstantDoubleInfo {}

struct ConstantClassInfo {}

impl ConstantClassInfo {
    fn new(reader: &mut ClassReader, cp: &ConstantPool) -> ConstantClassInfo {}
}

impl ConstantInfo for ConstantClassInfo {}

struct ConstantStringInfo {}

impl ConstantStringInfo {
    fn new(reader: &mut ClassReader, cp: &ConstantPool) -> ConstantStringInfo {}
}

impl ConstantInfo for ConstantStringInfo {}

struct ConstantMemberrefInfo {}

impl ConstantMemberrefInfo {
    fn new(reader: &mut ClassReader, cp: &ConstantPool) -> ConstantMemberrefInfo {}
}
//
//impl ConstantInfo for ConstantMemberrefInfo {
//}

struct ConstantFieldrefInfo {}

impl ConstantFieldrefInfo {
    fn new(reader: &mut ClassReader, info: ConstantMemberrefInfo) -> ConstantFieldrefInfo {}
}

impl ConstantInfo for ConstantFieldrefInfo {}

struct ConstantMethodrefInfo {}

impl ConstantMethodrefInfo {
    fn new(reader: &mut ClassReader, info: ConstantMemberrefInfo) -> ConstantMethodrefInfo {}
}

impl ConstantInfo for ConstantMethodrefInfo {}

struct ConstantInterfaceMethodrefInfo {}

impl ConstantInterfaceMethodrefInfo {
    fn new(reader: &mut ClassReader, info: ConstantMemberrefInfo) -> ConstantInterfaceMethodrefInfo {}
}

impl ConstantInfo for ConstantInterfaceMethodrefInfo {}

struct ConstantNameAndTypeInfo {}

impl ConstantNameAndTypeInfo {
    fn new(reader: &mut ClassReader) -> ConstantNameAndTypeInfo {}
}

impl ConstantInfo for ConstantNameAndTypeInfo {}

struct ConstantMethodHandleInfo {}

impl ConstantMethodHandleInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodHandleInfo {}
}

impl ConstantInfo for ConstantMethodHandleInfo {}

struct ConstantMethodTypeInfo {}

impl ConstantMethodTypeInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodTypeInfo {}
}

impl ConstantInfo for ConstantMethodTypeInfo {}

struct ConstantInvokeDynamicInfo {}

impl ConstantInvokeDynamicInfo {
    fn new(reader: &mut ClassReader) -> ConstantInvokeDynamicInfo {}
}

impl ConstantInfo for ConstantInvokeDynamicInfo {}