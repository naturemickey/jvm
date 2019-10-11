#[allow(non_camel_case_types)]
pub struct ISTORE_1 {}

impl Instruction for ISTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        ISTORE::_istore(frame, 1);
    }
}
impl Debug for ISTORE_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}