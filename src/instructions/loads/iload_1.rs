#[allow(non_camel_case_types)]
pub struct ILOAD_1 {}

impl Instruction for ILOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        ILOAD::_iload(frame, 1)
    }
}

impl Debug for ILOAD_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}