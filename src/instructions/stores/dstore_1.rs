#[allow(non_camel_case_types)]
struct DSTORE_1 {}

impl Instruction for DSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        DSTORE::_dstore(frame, 1);
    }
}