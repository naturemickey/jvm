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