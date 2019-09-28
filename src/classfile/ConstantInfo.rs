const CONSTANT_Class: u8 = 7;
const CONSTANT_Fieldref: u8 = 9;
const CONSTANT_Methodref: u8 = 10;
const CONSTANT_InterfaceMethodref: u8 = 11;
const CONSTANT_String: u8 = 8;
const CONSTANT_Integer: u8 = 3;
const CONSTANT_Float: u8 = 4;
const CONSTANT_Long: u8 = 5;
const CONSTANT_Double: u8 = 6;
const CONSTANT_NameAndType: u8 = 12;
const CONSTANT_Utf8: u8 = 1;
const CONSTANT_MethodHandle: u8 = 15;
const CONSTANT_MethodType: u8 = 16;
const CONSTANT_InvokeDynamic: u8 = 18;

fn read_constant_info(reader: &mut ClassReader, cp: &ConstantPool) -> Box<dyn ConstantInfo> {
    let tag = reader.read_u8();
    new_constant_info(tag, reader, cp)
}

fn new_constant_info(tag: u8, reader: &mut ClassReader, cp: &ConstantPool) -> Box<dyn ConstantInfo> {
    let mut c: dyn ConstantInfo = match tag {
        CONSTANT_Utf8 => ConstantUtf8Info::new(),
        CONSTANT_Integer => ConstantIntegerInfo::new(),
        CONSTANT_Float => ConstantFloatInfo::new(),
        CONSTANT_Long => ConstantLongInfo::new(),
        CONSTANT_Double => ConstantDoubleInfo::new(),
        CONSTANT_Class => ConstantClassInfo::new(cp),
        CONSTANT_String => ConstantStringInfo::new(cp),
        CONSTANT_Fieldref => ConstantFieldrefInfo::new(ConstantMemberrefInfo::new(cp)),
        CONSTANT_Methodref => ConstantMethodrefInfo::new(ConstantMemberrefInfo::new(cp)),
        CONSTANT_InterfaceMethodref => ConstantInterfaceMethodrefInfo::new(ConstantMemberrefInfo::new(cp)),
        CONSTANT_NameAndType => ConstantNameAndTypeInfo::new(),
        CONSTANT_MethodHandle => ConstantMethodHandleInfo::new(),
        CONSTANT_MethodType => ConstantMethodTypeInfo::new(),
        CONSTANT_InvokeDynamic => ConstantInvokeDynamicInfo::new(),
        _ => panic!("java. lang. ClassFormatError: constant pool tag!"),
    };
    c.read_info(reader);
    Box::new(c)
}

trait ConstantInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<dyn ConstantInfo>;
}

struct ConstantEmptyInfo {}
impl ConstantInfo for ConstantEmptyInfo{
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<ConstantInfo> {
        unimplemented!()
    }
}
const EMPTY_CONSTANT_INFO: Box<ConstantEmptyInfo> = Box::new(ConstantEmptyInfo{});

struct ConstantIntegerInfo {}

impl ConstantIntegerInfo {
    fn new() -> ConstantIntegerInfo {}
}

impl ConstantInfo for ConstantIntegerInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<dyn ConstantInfo> {}
}

struct ConstantFloatInfo {}

impl ConstantFloatInfo {
    fn new() -> ConstantFloatInfo {}
}

impl ConstantInfo for ConstantFloatInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<dyn ConstantInfo> {}
}

struct ConstantUtf8Info {}

impl ConstantUtf8Info {
    fn new() -> ConstantUtf8Info {}
}

impl ConstantInfo for ConstantUtf8Info {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<dyn ConstantInfo> {}
}

struct ConstantLongInfo {}

impl ConstantLongInfo {
    fn new() -> ConstantLongInfo {}
}

impl ConstantInfo for ConstantLongInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<dyn ConstantInfo> {}
}

struct ConstantDoubleInfo {}

impl ConstantDoubleInfo {
    fn new() -> ConstantDoubleInfo {}
}

impl ConstantInfo for ConstantDoubleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<dyn ConstantInfo> {}
}

struct ConstantClassInfo {}

impl ConstantClassInfo {
    fn new(cp: &ConstantPool) -> ConstantClassInfo {}
}

impl ConstantInfo for ConstantClassInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<dyn ConstantInfo> {}
}

struct ConstantStringInfo {}

impl ConstantStringInfo {
    fn new(cp: &ConstantPool) -> ConstantStringInfo {}
}

impl ConstantInfo for ConstantStringInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<ConstantInfo> {
        unimplemented!()
    }
}

struct ConstantMemberrefInfo {}

//impl ConstantMemberrefInfo {
//    fn new(cp: &ConstantPool) -> ConstantMemberrefInfo {}
//}
//
//impl ConstantInfo for ConstantMemberrefInfo {
//    fn read_info(&mut self, reader: &mut ClassReader) -> Box<ConstantInfo> {
//        unimplemented!()
//    }
//}

struct ConstantFieldrefInfo {}

impl ConstantFieldrefInfo {
    fn new(info: ConstantMemberrefInfo) -> ConstantFieldrefInfo {}
}

impl ConstantInfo for ConstantFieldrefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<ConstantInfo> {
        unimplemented!()
    }
}

struct ConstantMethodrefInfo {}

impl ConstantMethodrefInfo {
    fn new(info: ConstantMemberrefInfo) -> ConstantMethodrefInfo {}
}

impl ConstantInfo for ConstantMethodrefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<ConstantInfo> {
        unimplemented!()
    }
}

struct ConstantInterfaceMethodrefInfo {}

impl ConstantInterfaceMethodrefInfo {
    fn new(info: ConstantMemberrefInfo) -> ConstantInterfaceMethodrefInfo {}
}

impl ConstantInfo for ConstantInterfaceMethodrefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<ConstantInfo> {
        unimplemented!()
    }
}

struct ConstantNameAndTypeInfo {}

impl ConstantNameAndTypeInfo {
    fn new() -> ConstantNameAndTypeInfo {}
}

impl ConstantInfo for ConstantNameAndTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<ConstantInfo> {
        unimplemented!()
    }
}

struct ConstantMethodHandleInfo {}

impl ConstantMethodHandleInfo {
    fn new() -> ConstantMethodHandleInfo {}
}

impl ConstantInfo for ConstantMethodHandleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<ConstantInfo> {
        unimplemented!()
    }
}

struct ConstantMethodTypeInfo {}

impl ConstantMethodTypeInfo {
    fn new() -> ConstantMethodTypeInfo {}
}

impl ConstantInfo for ConstantMethodTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<ConstantInfo> {
        unimplemented!()
    }
}

struct ConstantInvokeDynamicInfo {}

impl ConstantInvokeDynamicInfo {
    fn new() -> ConstantInvokeDynamicInfo {}
}

impl ConstantInfo for ConstantInvokeDynamicInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<ConstantInfo> {
        unimplemented!()
    }
}