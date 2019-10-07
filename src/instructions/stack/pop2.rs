struct POP2 {}

impl Instruction for POP2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        stack.pop_slot();
        stack.pop_slot();
    }
}