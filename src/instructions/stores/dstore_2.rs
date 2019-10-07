struct DSTORE_2 {}

impl Instruction for DSTORE_2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        DSTORE::_dstore(frame, 2);
    }
}