#[allow(non_camel_case_types)]
pub struct F2I {}

impl Instruction for F2I {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let f = stack.pop_float();
        let i = f as i32;
        stack.push_int(i);
    }
}

impl Debug for F2I {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}