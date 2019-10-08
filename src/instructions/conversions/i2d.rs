struct I2D {}

impl Instruction for I2D {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let d = i as f64;
        stack.push_double(d);
    }
}