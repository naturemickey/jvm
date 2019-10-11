#[allow(non_camel_case_types)]
pub struct ASTORE_2 {}

impl Instruction for ASTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        ASTORE::_astore(frame, 2);
    }
}

impl Debug for ASTORE_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}