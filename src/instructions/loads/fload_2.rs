#[allow(non_camel_case_types)]
struct FLOAD_2 {}

impl Instruction for FLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        FLOAD::_fload(frame, 2)
    }
}