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
const CONSTANT_DYNAMIC_____________: u8 = 17;
const CONSTANT_INVOKE_DYNAMIC______: u8 = 18;
const CONSTANT_MODULE______________: u8 = 19;
const CONSTANT_PACKAGE_____________: u8 = 20;

pub enum ConstantInfo {
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
    Dynamic(ConstantDynamicInfo),
    InvokeDynamic(ConstantInvokeDynamicInfo),
    Module(ConstantModuleInfo),
    Package(ConstantPackageInfo),
}

impl ConstantInfo {
    fn read_constant_info(reader: &mut ClassReader, cp: Arc<ConstantPool>) -> ConstantInfo {
        let tag = reader.read_u8();
        Self::new_constant_info(tag, reader, cp.clone())
    }

    fn new_constant_info(tag: u8, reader: &mut ClassReader, cp: Arc<ConstantPool>) -> ConstantInfo {
        match tag {
            CONSTANT_UTF8________________ => Self::Utf8(ConstantUtf8Info::new(reader)),
            CONSTANT_INTEGER_____________ => Self::Integer(ConstantIntegerInfo::new(reader)),
            CONSTANT_FLOAT_______________ => Self::Float(ConstantFloatInfo::new(reader)),
            CONSTANT_LONG________________ => Self::Long(ConstantLongInfo::new(reader)),
            CONSTANT_DOUBLE______________ => Self::Double(ConstantDoubleInfo::new(reader)),
            CONSTANT_CLASS_______________ => Self::Class(ConstantClassInfo::new(reader, cp.clone())),
            CONSTANT_STRING______________ => Self::String(ConstantStringInfo::new(reader, cp.clone())),
            CONSTANT_FIELD_REF___________ => Self::FieldRef(ConstantFieldrefInfo::new(reader, cp.clone())),
            CONSTANT_METHOD_REF__________ => Self::MethodRef(ConstantMethodrefInfo::new(reader)),
            CONSTANT_INTERFACE_METHOD_REF => Self::InterfaceMethodRef(ConstantInterfaceMethodrefInfo::new(reader)),
            CONSTANT_NAME_AND_TYPE_______ => Self::NameAndType(ConstantNameAndTypeInfo::new(reader)),
            CONSTANT_METHOD_HANDLE_______ => Self::MethodHandle(ConstantMethodHandleInfo::new(reader)),
            CONSTANT_METHOD_TYPE_________ => Self::MethodType(ConstantMethodTypeInfo::new(reader)),
            CONSTANT_DYNAMIC_____________ => Self::Dynamic(ConstantDynamicInfo::new(reader)),
            CONSTANT_INVOKE_DYNAMIC______ => Self::InvokeDynamic(ConstantInvokeDynamicInfo::new(reader)),
            CONSTANT_MODULE______________ => Self::Module(ConstantModuleInfo::new(reader)),
            CONSTANT_PACKAGE_____________ => Self::Package(ConstantPackageInfo::new(reader)),
            _ => panic!("java.lang.ClassFormatError: constant pool tag {}", tag),
        }
    }
}
