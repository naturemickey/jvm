#[allow(non_camel_case_types)]
struct LLOAD_0 {}

impl Instruction for LLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        LLOAD::_lload(frame, 0)
    }
}