#[allow(non_camel_case_types)]
pub struct ILOAD_0 {}

impl Instruction for ILOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        ILOAD::_iload(frame, 0)
    }
}

impl Debug for ILOAD_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}