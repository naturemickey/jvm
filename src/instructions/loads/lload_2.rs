#[allow(non_camel_case_types)]
pub struct LLOAD_2 {}

impl Instruction for LLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        LLOAD::_lload(frame, 2)
    }
}