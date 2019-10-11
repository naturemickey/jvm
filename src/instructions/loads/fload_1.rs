#[allow(non_camel_case_types)]
pub struct FLOAD_1 {}

impl Instruction for FLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        FLOAD::_fload(frame, 1)
    }
}

impl Debug for FLOAD_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}