struct ASTORE_1 {}

impl Instruction for ASTORE_1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        ASTORE::_astore(frame, 1);
    }
}