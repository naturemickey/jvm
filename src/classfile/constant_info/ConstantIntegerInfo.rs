pub struct ConstantIntegerInfo {
    value: i32,
}

impl ConstantIntegerInfo {
    fn new(reader: &mut ClassReader) -> ConstantIntegerInfo {
        Self { value: reader.read_u32() as i32 }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}