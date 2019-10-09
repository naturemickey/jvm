#[allow(non_camel_case_types)]
struct LSTORE_2 {}

impl Instruction for LSTORE_2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        LSTORE::_lstore(frame, 2);
    }
}