struct BranchInstruction {
    off_set: i16
}

impl Instruction for BranchInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.off_set = reader.read_i16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        unimplemented!()
    }
}