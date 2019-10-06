struct NoOperandsInstruction {}

impl Instruction for NoOperandsInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        // nothing to do.
    }
}