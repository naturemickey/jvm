#[allow(non_camel_case_types)]
struct ISTORE_2 {}

impl Instruction for ISTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        ISTORE::_istore(frame, 2);
    }
}