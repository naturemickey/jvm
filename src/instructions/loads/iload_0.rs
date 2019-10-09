#[allow(non_camel_case_types)]
struct ILOAD_0 {}

impl Instruction for ILOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        ILOAD::_iload(frame, 0)
    }
}