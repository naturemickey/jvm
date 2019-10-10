pub struct SourceFileAttribute {
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