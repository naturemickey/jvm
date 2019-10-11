#[allow(non_camel_case_types)]
pub struct ISTORE_2 {}

impl Instruction for ISTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        ISTORE::_istore(frame, 2);
    }
}
impl Debug for ISTORE_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}