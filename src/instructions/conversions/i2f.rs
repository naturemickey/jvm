#[allow(non_camel_case_types)]
pub struct I2F {}

impl Instruction for I2F {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let f = i as f32;
        stack.push_float(f);
    }
}

impl Debug for I2F {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}