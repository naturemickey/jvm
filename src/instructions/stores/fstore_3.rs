struct FSTORE_3 {}

impl Instruction for FSTORE_3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        FSTORE::_fstore(frame, 3);
    }
}