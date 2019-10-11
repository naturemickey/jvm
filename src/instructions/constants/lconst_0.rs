#[allow(non_camel_case_types)]
pub struct LCONST_0 {}

impl Instruction for LCONST_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_long(0);
    }
}

impl Debug for LCONST_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}