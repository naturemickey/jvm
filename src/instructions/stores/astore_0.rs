#[allow(non_camel_case_types)]
pub struct ASTORE_0 {}

impl Instruction for ASTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        ASTORE::_astore(frame, 0);
    }
}

impl Debug for ASTORE_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}