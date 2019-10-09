#[allow(non_camel_case_types)]
struct I2L {}

impl Instruction for I2L {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let l = i as i64;
        stack.push_long(l);
    }
}