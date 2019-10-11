#[allow(non_camel_case_types)]
pub struct INEG {}

impl Instruction for INEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let val = stack.pop_int();
        stack.push_int(-val);
    }
}

impl Debug for INEG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}