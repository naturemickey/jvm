#[allow(non_camel_case_types)]
pub struct ICONST_3 {}

impl Instruction for ICONST_3 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(3);
    }
}

impl Debug for ICONST_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}