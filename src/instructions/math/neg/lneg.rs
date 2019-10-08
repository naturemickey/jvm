struct LNEG {}

impl Instruction for LNEG {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let val = stack.pop_long();
        stack.push_long(-val);
    }
}