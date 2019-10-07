struct ISTORE_3 {}

impl Instruction for ISTORE_3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        ISTORE::_istore(frame, 3);
    }
}