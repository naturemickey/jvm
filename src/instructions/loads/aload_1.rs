#[allow(non_camel_case_types)]
struct ALOAD_1 {}

impl Instruction for ALOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        ALOAD::_aload(frame, 1)
    }
}