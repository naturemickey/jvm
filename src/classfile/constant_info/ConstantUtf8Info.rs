struct ConstantUtf8Info {
    string: String,
}

impl ConstantUtf8Info {
    fn new(reader: &mut ClassReader) -> ConstantUtf8Info {
        let length = reader.read_u16();
        let bytes = reader.read_bytes(length as u32);
        let string = String::from_utf8(bytes).unwrap();
        Self { string }
    }
}