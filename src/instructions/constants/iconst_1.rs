struct ICOUNT_1 {}

impl Instruction for ICOUNT_1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(1);
    }
}