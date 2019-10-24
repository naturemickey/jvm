pub struct ConstantPackageInfo {
    name_index: u16
}

impl ConstantPackageInfo {
    fn new(reader: &mut ClassReader) -> ConstantPackageInfo {
        Self { name_index: reader.read_u16() }
    }
}