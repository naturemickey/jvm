struct LCMP {}

impl Instruction for LCMP {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        stack.push_int(
            if v1 > v2 {
                1
            } else if v1 == v2 {
                0
            } else {
                -1
            });
    }
}