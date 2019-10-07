struct ALOAD_0 {}

impl Instruction for ALOAD_0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        ALOAD::_aload(frame, 0)
    }
}