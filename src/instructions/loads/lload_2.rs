#[allow(non_camel_case_types)]
struct LLOAD_2 {}

impl Instruction for LLOAD_2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        LLOAD::_lload(frame, 2)
    }
}