struct ASTORE_2 {}

impl Instruction for ASTORE_2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        ASTORE::_astore(frame, 2);
    }
}