struct D2F {}

impl Instruction for D2F {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let d = stack.pop_double();
        let f = d as f32;
        stack.push_float(f);
    }
}