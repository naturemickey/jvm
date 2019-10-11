#[allow(non_camel_case_types)]
pub struct FCONST_1 {}

impl Instruction for FCONST_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_float(1f32);
    }
}

impl Debug for FCONST_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}