#[allow(non_camel_case_types)]
struct ALOAD_3 {}

impl Instruction for ALOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        ALOAD::_aload(frame, 3)
    }
}