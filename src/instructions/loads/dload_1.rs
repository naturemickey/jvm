#[allow(non_camel_case_types)]
struct DLOAD_1 {}

impl Instruction for DLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        DLOAD::_dload(frame, 1)
    }
}