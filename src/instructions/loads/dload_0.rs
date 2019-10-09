#[allow(non_camel_case_types)]
struct DLOAD_0 {}

impl Instruction for DLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        DLOAD::_dload(frame, 0)
    }
}