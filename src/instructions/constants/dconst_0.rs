#[allow(non_camel_case_types)]
pub struct DCONST_0 {}

impl Instruction for DCONST_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_double(0f64);
    }
}

impl Debug for DCONST_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}