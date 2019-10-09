#[allow(non_camel_case_types)]
struct ISTORE_1 {}

impl Instruction for ISTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        ISTORE::_istore(frame, 1);
    }
}