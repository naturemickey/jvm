struct FCOUNT_2 {}

impl Instruction for FCOUNT_2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_float(2f32);
    }
}