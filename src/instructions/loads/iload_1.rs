#[allow(non_camel_case_types)]
struct ILOAD_1 {}

impl Instruction for ILOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        ILOAD::_iload(frame, 1)
    }
}