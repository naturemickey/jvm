#[allow(non_camel_case_types)]
pub struct FLOAD_3 {}

impl Instruction for FLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        FLOAD::_fload(frame, 3)
    }
}