struct ConstantLongInfo {
    value: i64,
}

impl ConstantLongInfo {
    fn new(reader: &mut ClassReader) -> ConstantLongInfo {
        Self { value: reader.read_u64() as i64 }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}