struct ConstantDoubleInfo {
    float64: f64,
}

impl ConstantDoubleInfo {
    fn new(reader: &mut ClassReader) -> ConstantDoubleInfo {
        Self { float64: f64::from_bits(reader.read_u64()) }
    }
}