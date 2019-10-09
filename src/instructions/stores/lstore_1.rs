#[allow(non_camel_case_types)]
struct LSTORE_1 {}

impl Instruction for LSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        LSTORE::_lstore(frame, 1);
    }
}