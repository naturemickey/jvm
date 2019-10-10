#[allow(non_camel_case_types)]
pub struct I2D {}

impl Instruction for I2D {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let d = i as f64;
        stack.push_double(d);
    }
}