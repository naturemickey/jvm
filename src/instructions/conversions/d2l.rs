#[allow(non_camel_case_types)]
pub struct D2L {}

impl Instruction for D2L {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let d = stack.pop_double();
        let l = d as i64;
        stack.push_long(l);
    }
}

impl Debug for D2L {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}