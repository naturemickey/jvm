#[allow(non_camel_case_types)]
pub struct ICONST_5 {}

impl Instruction for ICONST_5 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(5);
    }
}

impl Debug for ICONST_5 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}