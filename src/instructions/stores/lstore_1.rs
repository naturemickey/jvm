#[allow(non_camel_case_types)]
pub struct LSTORE_1 {}

impl Instruction for LSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        LSTORE::_lstore(frame, 1);
    }
}
impl Debug for LSTORE_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}