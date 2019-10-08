struct F2D {}

impl Instruction for F2D {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let f = stack.pop_float();
        let d = f as f64;
        stack.push_double(d);
    }
}