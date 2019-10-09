#[allow(non_camel_case_types)]
struct I2C {}

impl Instruction for I2C {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let c = (i as u16) as i32;
        stack.push_int(c);
    }
}