#[allow(non_camel_case_types)]
struct DSTORE_2 {}

impl Instruction for DSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        DSTORE::_dstore(frame, 2);
    }
}