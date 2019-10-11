#[allow(non_camel_case_types)]
pub struct FCONST_0 {}

impl Instruction for FCONST_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_float(0f32);
    }
}

impl Debug for FCONST_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}