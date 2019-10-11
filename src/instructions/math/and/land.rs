#[allow(non_camel_case_types)]
pub struct LAND {}

impl Instruction for LAND {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v1 = stack.pop_long();
        let v2 = stack.pop_long();
        stack.push_long(v1 & v2);
    }
}

impl Debug for LAND {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}