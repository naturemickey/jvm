#[allow(non_camel_case_types)]
struct DSTORE_3 {}

impl Instruction for DSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        DSTORE::_dstore(frame, 3);
    }
}