struct BranchInstruction {
    off_set: i16
}

impl BranchInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.off_set = reader.read_i16();
    }
}
