#[allow(non_camel_case_types)]
pub struct DSTORE_1 {}

impl Instruction for DSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        DSTORE::_dstore(frame, 1);
    }
}

impl Debug for DSTORE_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}