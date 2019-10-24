pub struct ConstantFloatInfo {
    value: f32,
}

impl ConstantFloatInfo {
    fn new(reader: &mut ClassReader) -> ConstantFloatInfo {
        Self { value: f32::from_bits(reader.read_u32()) }
    }
    pub fn value(&self) -> f32 {
        self.value
    }
}