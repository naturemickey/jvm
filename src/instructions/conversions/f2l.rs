struct F2L {}

impl Instruction for F2L {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let f = stack.pop_float();
        let l = f as i64;
        stack.push_long(l);
    }
}