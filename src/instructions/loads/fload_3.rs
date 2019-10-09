#[allow(non_camel_case_types)]
struct FLOAD_3 {}

impl Instruction for FLOAD_3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        FLOAD::_fload(frame, 3)
    }
}