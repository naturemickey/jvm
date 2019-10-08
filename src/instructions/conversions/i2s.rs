struct I2S {}

impl Instruction for I2S {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let s = (i as i16) as i32;
        stack.push_int(s);
    }
}