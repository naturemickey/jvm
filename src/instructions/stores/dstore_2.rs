#[allow(non_camel_case_types)]
pub struct DSTORE_2 {}

impl Instruction for DSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        DSTORE::_dstore(frame, 2);
    }
}

impl Debug for DSTORE_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}