#[allow(non_camel_case_types)]
pub struct FSTORE_1 {}

impl Instruction for FSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        FSTORE::_fstore(frame, 1);
    }
}

impl Debug for FSTORE_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}