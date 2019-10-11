#[allow(non_camel_case_types)]
pub struct D2I {}

impl Instruction for D2I {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let d = stack.pop_double();
        let i = d as i32;
        stack.push_int(i);
    }
}

impl Debug for D2I {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}