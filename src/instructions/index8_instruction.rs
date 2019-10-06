struct Index8Instruction {
    index: u8
}

impl Instruction for Index8Instruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        unimplemented!()
    }
}