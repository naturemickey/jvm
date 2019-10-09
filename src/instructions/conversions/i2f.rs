#[allow(non_camel_case_types)]
struct I2F {}

impl Instruction for I2F {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let f = i as f32;
        stack.push_float(f);
    }
}