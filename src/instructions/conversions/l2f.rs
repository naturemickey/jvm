#[allow(non_camel_case_types)]
pub struct L2F {}

impl Instruction for L2F {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let l = stack.pop_long();
        let f = l as f32;
        stack.push_float(f);
    }
}

impl Debug for L2F {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}