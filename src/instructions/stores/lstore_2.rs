#[allow(non_camel_case_types)]
pub struct LSTORE_2 {}

impl Instruction for LSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        LSTORE::_lstore(frame, 2);
    }
}
impl Debug for LSTORE_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}