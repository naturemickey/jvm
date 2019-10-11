#[allow(non_camel_case_types)]
pub struct ISTORE_3 {}

impl Instruction for ISTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        ISTORE::_istore(frame, 3);
    }
}
impl Debug for ISTORE_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}