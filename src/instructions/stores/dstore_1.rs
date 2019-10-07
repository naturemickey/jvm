struct DSTORE_1 {}

impl Instruction for DSTORE_1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        DSTORE::_dstore(frame, 1);
    }
}