#[allow(non_camel_case_types)]
struct FLOAD_1 {}

impl Instruction for FLOAD_1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        FLOAD::_fload(frame, 1)
    }
}