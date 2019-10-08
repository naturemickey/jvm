struct I2L {}

impl Instruction for I2L {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let l = i as i64;
        stack.push_long(l);
    }
}