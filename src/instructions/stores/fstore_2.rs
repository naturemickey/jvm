#[allow(non_camel_case_types)]
struct FSTORE_2 {}

impl Instruction for FSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        FSTORE::_fstore(frame, 2);
    }
}