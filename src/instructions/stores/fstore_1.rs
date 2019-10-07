struct FSTORE_1 {}

impl Instruction for FSTORE_1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        FSTORE::_fstore(frame, 1);
    }
}