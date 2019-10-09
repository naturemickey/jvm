#[allow(non_camel_case_types)]
struct DLOAD_3 {}

impl Instruction for DLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        DLOAD::_dload(frame, 3)
    }
}