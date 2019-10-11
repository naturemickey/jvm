#[allow(non_camel_case_types)]
pub struct I2C {}

impl Instruction for I2C {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let i = stack.pop_int();
        let c = (i as u16) as i32;
        stack.push_int(c);
    }
}

impl Debug for I2C {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}