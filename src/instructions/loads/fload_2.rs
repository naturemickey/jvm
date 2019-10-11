#[allow(non_camel_case_types)]
pub struct FLOAD_2 {}

impl Instruction for FLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        FLOAD::_fload(frame, 2)
    }
}

impl Debug for FLOAD_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}