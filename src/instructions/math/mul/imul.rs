#[allow(non_camel_case_types)]
pub struct IMUL {}

impl Instruction for IMUL {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        stack.push_int(v1 * v2);
    }
}

impl Debug for IMUL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}