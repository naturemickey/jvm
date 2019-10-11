#[allow(non_camel_case_types)]
pub struct ALOAD_3 {}

impl Instruction for ALOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        ALOAD::_aload(frame, 3)
    }
}

impl Debug for ALOAD_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}