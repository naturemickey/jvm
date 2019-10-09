#[allow(non_camel_case_types)]
struct ASTORE_3 {}

impl Instruction for ASTORE_3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        ASTORE::_astore(frame, 3);
    }
}