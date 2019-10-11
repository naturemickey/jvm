#[allow(non_camel_case_types)]
pub struct DCONST_1 {}

impl Instruction for DCONST_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_double(1f64);
    }
}

impl Debug for DCONST_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}