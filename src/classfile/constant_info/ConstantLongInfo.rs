struct ConstantLongInfo {
    integer64: i64,
}

impl ConstantLongInfo {
    fn new(reader: &mut ClassReader) -> ConstantLongInfo {
        Self { integer64: reader.read_u64() as i64 }
    }
}