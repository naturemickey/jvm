#[allow(non_camel_case_types)]
struct DLOAD_2 {}

impl Instruction for DLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        DLOAD::_dload(frame, 2)
    }
}