pub struct ConstantStringInfo {
    string_index: u16,
    cp: Arc<ConstantPool>,
}

impl ConstantStringInfo {
    fn new(reader: &mut ClassReader, cp: Arc<ConstantPool>) -> ConstantStringInfo {
        let string_index = reader.read_u16();
        Self { string_index, cp }
    }

    pub fn string<'a>(&'a self) -> &'a str {
        self.cp.get_utf8(self.string_index)
    }
}