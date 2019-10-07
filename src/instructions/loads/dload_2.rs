struct DLOAD_2 {}

impl Instruction for DLOAD_2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        DLOAD::_dload(frame, 2)
    }
}