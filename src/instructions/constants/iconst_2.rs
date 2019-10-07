struct ICOUNT_2 {}

impl Instruction for ICOUNT_2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(2);
    }
}