#[allow(non_camel_case_types)]
pub struct DSTORE_0 {}

impl Instruction for DSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        DSTORE::_dstore(frame, 0);
    }
}

impl Debug for DSTORE_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}