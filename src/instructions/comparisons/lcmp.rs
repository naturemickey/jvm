#[allow(non_camel_case_types)]
pub struct LCMP {}

impl Instruction for LCMP {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        stack.push_int(
            if v1 > v2 {
                1
            } else if v1 == v2 {
                0
            } else {
                -1
            });
    }
}

impl Debug for LCMP {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}