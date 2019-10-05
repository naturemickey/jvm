struct ConstantFloatInfo {
    float32: f32,
}

impl ConstantFloatInfo {
    fn new(reader: &mut ClassReader) -> ConstantFloatInfo {
        Self { float32: f32::from_bits(reader.read_u32()) }
    }
}