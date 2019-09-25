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
        CONSTANT_Integer => ConstantIntegerInfo::new(),
        // TODO
        _ => panic!("impossible constant tag."),
    };
    c.read_info(reader);
    Box::new(c)
}

trait ConstantInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<dyn ConstantInfo>;
}

struct ConstantIntegerInfo {}

impl ConstantIntegerInfo {
    fn new() -> ConstantIntegerInfo {}
}

impl ConstantInfo for ConstantIntegerInfo {
    fn read_info(&mut self, reader: &mut ClassReader) -> Box<dyn ConstantInfo> {}
}

struct ConstantFloatInfo {}
