pub struct ConstantValueAttribute {
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