struct LCOUNT_1 {}

impl Instruction for LCOUNT_1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_long(1);
    }
}