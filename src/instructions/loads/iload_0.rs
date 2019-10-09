#[allow(non_camel_case_types)]
struct ILOAD_0 {}

impl Instruction for ILOAD_0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        ILOAD::_iload(frame, 0)
    }
}