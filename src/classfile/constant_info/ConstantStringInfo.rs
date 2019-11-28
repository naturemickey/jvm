pub struct ConstantStringInfo {
    string_index: u16,
    cp: Arc<RwLock<ConstantPool>>,
}

impl ConstantStringInfo {
    fn new(reader: &mut ClassReader, cp: Arc<RwLock<ConstantPool>>) -> ConstantStringInfo {
        let string_index = reader.read_u16();
        Self { string_index, cp }
    }

    pub fn string(&self) -> Arc<String> {
        self.cp.read().unwrap().get_utf8(self.string_index)
    }
}