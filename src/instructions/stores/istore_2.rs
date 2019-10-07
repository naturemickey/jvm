struct ISTORE_2 {}

impl Instruction for ISTORE_2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        ISTORE::_istore(frame, 2);
    }
}