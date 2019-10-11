#[allow(non_camel_case_types)]
pub struct ICONST_1 {}

impl Instruction for ICONST_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(1);
    }
}

impl Debug for ICONST_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}