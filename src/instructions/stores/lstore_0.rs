#[allow(non_camel_case_types)]
struct LSTORE_0 {}

impl Instruction for LSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        LSTORE::_lstore(frame, 0);
    }
}