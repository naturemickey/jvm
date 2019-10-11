#[allow(non_camel_case_types)]
pub struct FSTORE_3 {}

impl Instruction for FSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        FSTORE::_fstore(frame, 3);
    }
}

impl Debug for FSTORE_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}