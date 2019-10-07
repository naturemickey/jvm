struct ISTORE_0 {}

impl Instruction for ISTORE_0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        ISTORE::_istore(frame, 0);
    }
}