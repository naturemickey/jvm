#[allow(non_camel_case_types)]
pub struct DLOAD_2 {}

impl Instruction for DLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        DLOAD::_dload(frame, 2)
    }
}

impl Debug for DLOAD_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}