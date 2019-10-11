#[allow(non_camel_case_types)]
pub struct ILOAD_3 {}

impl Instruction for ILOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        ILOAD::_iload(frame, 3)
    }
}

impl Debug for ILOAD_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}