#[allow(non_camel_case_types)]
struct FLOAD_1 {}

impl Instruction for FLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        FLOAD::_fload(frame, 1)
    }
}