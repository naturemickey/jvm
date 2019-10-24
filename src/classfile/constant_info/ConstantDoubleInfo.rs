pub struct ConstantDoubleInfo {
    value: f64,
}

impl ConstantDoubleInfo {
    fn new(reader: &mut ClassReader) -> ConstantDoubleInfo {
        Self { value: f64::from_bits(reader.read_u64()) }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}