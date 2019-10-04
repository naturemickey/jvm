enum AttributeInfo {
    Code,
    ConstantValue,
    Deprecated,
    Exceptions,
    LineNumberTable,
    LocalVariableTable,
    SourceFile,
    Synthetic,
    Unparsed(AttributeUnparsedInfo),
}

impl AttributeInfo {
    fn read_attributes(reader: &mut ClassReader, cp: &ConstantPool) -> Vec<AttributeInfo> {
        let count = reader.read_u16();
        let mut attributes = Vec::new();
        for _ in 0..count {
            attributes.push(Self::read_attribute(reader, cp));
        }
        attributes
    }
    fn read_attribute(reader: &mut ClassReader, cp: &ConstantPool) -> AttributeInfo {
        let attr_name_index = reader.read_u16();
        let attr_name = cp.get_utf8(attr_name_index);
        let attr_len = reader.read_u32();
        Self::new(attr_name, attr_len, reader)
    }
    fn new(name: &str, length: u32, reader: &mut ClassReader) -> AttributeInfo {
        match name {
            "Code" => Self::Code,
            "ConstantValue" => Self::ConstantValue,
            "Deprecated" => Self::Deprecated,
            "Exceptions" => Self::Exceptions,
            "LineNumberTable" => Self::LineNumberTable,
            "LocalVariableTable" => Self::LocalVariableTable,
            "SourceFile" => Self::SourceFile,
            "Synthetic" => Self::Synthetic,
            _ => Self::Unparsed(AttributeUnparsedInfo::new(name, length, reader)),
        }
    }
}

struct AttributeUnparsedInfo {
    name: String,
    length: u32,
    info: Vec<u8>,
}

impl AttributeUnparsedInfo {
    fn new(name: &str, length: u32, reader: &mut ClassReader) -> AttributeUnparsedInfo {
        let info = reader.read_bytes(length);
        Self { name: name.to_string(), length, info }
    }
}