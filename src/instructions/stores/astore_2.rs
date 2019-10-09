#[allow(non_camel_case_types)]
struct ASTORE_2 {}

impl Instruction for ASTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        ASTORE::_astore(frame, 2);
    }
}