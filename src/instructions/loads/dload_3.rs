#[allow(non_camel_case_types)]
pub struct DLOAD_3 {}

impl Instruction for DLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        DLOAD::_dload(frame, 3)
    }
}

impl Debug for DLOAD_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}