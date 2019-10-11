#[allow(non_camel_case_types)]
pub struct LLOAD_0 {}

impl Instruction for LLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        LLOAD::_lload(frame, 0)
    }
}

impl Debug for LLOAD_0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}