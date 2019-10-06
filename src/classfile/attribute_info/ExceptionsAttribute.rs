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