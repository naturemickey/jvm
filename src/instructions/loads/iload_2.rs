#[allow(non_camel_case_types)]
pub struct ILOAD_2 {}

impl Instruction for ILOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        ILOAD::_iload(frame, 2)
    }
}

impl Debug for ILOAD_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}