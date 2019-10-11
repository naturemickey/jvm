#[allow(non_camel_case_types)]
pub struct ALOAD_2 {}

impl Instruction for ALOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        ALOAD::_aload(frame, 2)
    }
}

impl Debug for ALOAD_2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}