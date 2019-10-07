struct LLOAD_1 {}

impl Instruction for LLOAD_1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        LLOAD::_lload(frame, 1)
    }
}