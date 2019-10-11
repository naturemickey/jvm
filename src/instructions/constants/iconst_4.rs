#[allow(non_camel_case_types)]
pub struct ICONST_4 {}

impl Instruction for ICONST_4 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(4);
    }
}

impl Debug for ICONST_4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}