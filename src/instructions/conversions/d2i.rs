struct D2I {}

impl Instruction for D2I {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let d = stack.pop_double();
        let i = d as i32;
        stack.push_int(i);
    }
}