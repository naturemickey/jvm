struct LSTORE_0 {}

impl Instruction for LSTORE_0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        LSTORE::_lstore(frame, 0);
    }
}