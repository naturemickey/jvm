#[allow(non_camel_case_types)]
struct ALOAD_0 {}

impl Instruction for ALOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        ALOAD::_aload(frame, 0)
    }
}