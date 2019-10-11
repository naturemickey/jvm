#[allow(non_camel_case_types)]
pub struct DADD {}

impl Instruction for DADD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v1 = stack.pop_double();
        let v2 = stack.pop_double();
        stack.push_double(v1 + v2);
    }
}

impl Debug for DADD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}