pub enum AttributeInfo {
    Code(CodeAttribute),
    ConstantValue(ConstantValueAttribute),
    Deprecated(DeprecatedAttribute),
    Exceptions(ExceptionsAttribute),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    SourceFile(SourceFileAttribute),
    Synthetic(SyntheticAttribute),
    // todo
    Unparsed(UnparsedAttribute),
}

impl AttributeInfo {
    fn read_attributes(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> Vec<AttributeInfo> {
        let count = reader.read_u16();
        let mut attributes = Vec::new();
        for _ in 0..count {
            attributes.push(Self::read_attribute(reader, cp.clone()));
        }
        attributes
    }
    fn read_attribute(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> AttributeInfo {
        let attr_name_index = reader.read_u16();
        let attr_name = cp.read().unwrap().get_utf8(attr_name_index);
        let attr_len = reader.read_u32();
        Self::new(attr_name_index, attr_name, attr_len, reader, cp.clone())
    }
    fn new(name_index: u16, name: &str, length: u32, reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> AttributeInfo {
        match name {
            "Code" => Self::Code(CodeAttribute::new(reader, cp)),
            "ConstantValue" => Self::ConstantValue(ConstantValueAttribute::new(reader)),
            "Deprecated" => Self::Deprecated(DeprecatedAttribute {}),
            "Exceptions" => Self::Exceptions(ExceptionsAttribute::new(reader)),
            "LineNumberTable" => Self::LineNumberTable(LineNumberTableAttribute::new(reader)),
            "LocalVariableTable" => Self::LocalVariableTable(LocalVariableTableAttribute::new(reader)),
            "SourceFile" => Self::SourceFile(SourceFileAttribute::new(reader)),
            "Synthetic" => Self::Synthetic(SyntheticAttribute {}),
            _ => Self::Unparsed(UnparsedAttribute::new(name_index, length, reader)),
        }
    }
}
