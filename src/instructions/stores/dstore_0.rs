#[allow(non_camel_case_types)]
struct DSTORE_0 {}

impl Instruction for DSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        DSTORE::_dstore(frame, 0);
    }
}