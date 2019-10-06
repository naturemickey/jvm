struct Index16Instruction {
    index: u16
}

impl Instruction for Index16Instruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        unimplemented!()
    }
}