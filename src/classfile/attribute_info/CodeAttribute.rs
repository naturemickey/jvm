pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<AttributeInfo>,
}

impl CodeAttribute {
    fn new(reader: &mut ClassReader, cp: Arc<ConstantPool>) -> CodeAttribute {
        let max_stack = reader.read_u16();
        let max_locals = reader.read_u16();
        let code_length = reader.read_u32();
        let code = reader.read_bytes(code_length);
        let exception_table = ExceptionTableEntry::read_exception_table(reader);
        let attributes = AttributeInfo::read_attributes(reader, cp);
        Self { max_stack, max_locals, code, exception_table, attributes }
    }

    pub fn max_stack(&self) -> u16 { self.max_stack }
    pub fn max_locals(&self) -> u16 { self.max_locals }
    pub fn code(&self) -> &Vec<u8> { &self.code }
    pub fn exception_table(&self) -> &Vec<ExceptionTableEntry> { &self.exception_table }
    pub fn attributes(&self) -> &Vec<AttributeInfo> { &self.attributes }
}

pub struct ExceptionTableEntry {
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