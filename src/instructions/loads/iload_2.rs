#[allow(non_camel_case_types)]
pub struct ILOAD_2 {}

impl Instruction for ILOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        ILOAD::_iload(frame, 2)
    }
}