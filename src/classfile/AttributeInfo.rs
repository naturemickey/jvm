enum AttributeInfo {
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
        Self::new(attr_name_index, attr_name, attr_len, reader, cp)
    }
    fn new(name_index: u16, name: &str, length: u32, reader: &mut ClassReader, cp: &ConstantPool) -> AttributeInfo {
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

struct UnparsedAttribute {
    name_index: u16,
    length: u32,
    info: Vec<u8>,
}

impl UnparsedAttribute {
    fn new(name_index: u16, length: u32, reader: &mut ClassReader) -> UnparsedAttribute {
        let info = reader.read_bytes(length);
        Self { name_index, length, info }
    }
}

struct DeprecatedAttribute {}

struct SyntheticAttribute {}

struct SourceFileAttribute {
    name_index: u16
}

impl SourceFileAttribute {
    fn new(reader: &mut ClassReader) -> SourceFileAttribute {
        Self { name_index: reader.read_u16() }
    }
    fn file_name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        cp.get_utf8(self.name_index)
    }
}

struct ConstantValueAttribute {
    value_index: u16
}

impl ConstantValueAttribute {
    fn new(reader: &mut ClassReader) -> ConstantValueAttribute {
        Self { value_index: reader.read_u16() }
    }
    fn value_index(&self) -> u16 {
        self.value_index
    }
}

struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<AttributeInfo>,
}

impl CodeAttribute {
    fn new(reader: &mut ClassReader, cp: &ConstantPool) -> CodeAttribute {
        let max_stack = reader.read_u16();
        let max_locals = reader.read_u16();
        let code_length = reader.read_u32();
        let code = reader.read_bytes(code_length);
        let exception_table = ExceptionTableEntry::read_exception_table(reader);
        let attributes = AttributeInfo::read_attributes(reader, cp);
        Self { max_stack, max_locals, code, exception_table, attributes }
    }
}

struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionTableEntry {
    fn read_exception_table(reader: &mut ClassReader) -> Vec<ExceptionTableEntry> {
        let table_length = reader.read_u16();
        let mut table = Vec::new();
        for _ in 0..table_length {
            let start_pc = reader.read_u16();
            let end_pc = reader.read_u16();
            let handler_pc = reader.read_u16();
            let catch_type = reader.read_u16();
            table.push(Self { start_pc, end_pc, handler_pc, catch_type })
        }
        table
    }
}

struct ExceptionsAttribute {
    index_table: Vec<u16>
}

impl ExceptionsAttribute {
    fn new(reader: &mut ClassReader) -> ExceptionsAttribute {
        Self { index_table: reader.read_u16s() }
    }
    fn index_table(&self) -> &Vec<u16> {
        &self.index_table
    }
}

struct LineNumberTableAttribute {
    line_number_table: Vec<LineNumberTableEntry>
}

struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

impl LineNumberTableAttribute {
    fn new(reader: &mut ClassReader) -> LineNumberTableAttribute {
        let line_number_table_length = reader.read_u16();
        let mut line_number_table = Vec::new();
        for _ in 0..line_number_table_length {
            let start_pc = reader.read_u16();
            let line_number = reader.read_u16();
            line_number_table.push(LineNumberTableEntry { start_pc, line_number });
        }
        Self { line_number_table }
    }
}

struct LocalVariableTableAttribute {
    local_variable_table: Vec<LocalVariableTableEntry>
}

struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl LocalVariableTableAttribute {
    fn new(reader: &mut ClassReader) -> LocalVariableTableAttribute {
        let local_variable_table_length = reader.read_u16();
        let mut local_variable_table = Vec::new();
        for _ in 0..local_variable_table_length {
            let start_pc = reader.read_u16();
            let length = reader.read_u16();
            let name_index = reader.read_u16();
            let descriptor_index = reader.read_u16();
            let index = reader.read_u16();
            local_variable_table.push(LocalVariableTableEntry { start_pc, length, name_index, descriptor_index, index });
        }
        Self { local_variable_table }
    }
}