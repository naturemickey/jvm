#[allow(non_camel_case_types)]
pub struct RETURN {}

impl Instruction for RETURN {
    fn execute(&mut self, frame: &mut Frame) {
        frame.thread().write().unwrap().pop_frame();
    }
}

impl Debug for RETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}