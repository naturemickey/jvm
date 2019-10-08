struct I2B {}

impl Instruction for I2B {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let b = (i as i8) as i32;
        stack.push_int(b);
    }
}