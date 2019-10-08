struct IAND {}

impl Instruction for IAND {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v1 = stack.pop_int();
        let v2 = stack.pop_int();
        stack.push_int(v1 & v2);
    }
}