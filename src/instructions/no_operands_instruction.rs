struct NoOperandsInstruction {}

impl Instruction for NoOperandsInstruction {
    fn fetch_operands(reader: &BytecodeReader) {
        // do nothing
    }

    fn execute(frame: &Frame) {
        // do nothing
    }
}