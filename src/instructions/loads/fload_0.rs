struct FLOAD_0 {}

impl Instruction for FLOAD_0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        FLOAD::_fload(frame, 0)
    }
}