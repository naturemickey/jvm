#[allow(non_camel_case_types)]
struct ALOAD_2 {}

impl Instruction for ALOAD_2 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        ALOAD::_aload(frame, 2)
    }
}