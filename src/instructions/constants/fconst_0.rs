struct FCOUNT_0 {}

impl Instruction for FCOUNT_0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_float(0f32);
    }
}