#[allow(non_camel_case_types)]
pub struct LLOAD_1 {}

impl Instruction for LLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        LLOAD::_lload(frame, 1)
    }
}