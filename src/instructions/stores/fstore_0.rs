#[allow(non_camel_case_types)]
struct FSTORE_0 {}

impl Instruction for FSTORE_0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        FSTORE::_fstore(frame, 0);
    }
}