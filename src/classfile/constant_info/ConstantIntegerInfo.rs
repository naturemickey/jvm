struct ConstantIntegerInfo {
    integer32: i32,
}

impl ConstantIntegerInfo {
    fn new(reader: &mut ClassReader) -> ConstantIntegerInfo {
        Self { integer32: reader.read_u32() as i32 }
    }
}