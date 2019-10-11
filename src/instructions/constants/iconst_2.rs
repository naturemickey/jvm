#[allow(non_camel_case_types)]
pub struct ICONST_2 {}

impl Instruction for ICONST_2 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(2);
    }
}

impl Debug for ICONST_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}