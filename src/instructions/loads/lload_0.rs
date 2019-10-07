struct LLOAD_0 {}

impl Instruction for LLOAD_0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        LLOAD::_lload(frame, 0)
    }
}