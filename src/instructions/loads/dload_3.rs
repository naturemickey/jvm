#[allow(non_camel_case_types)]
struct DLOAD_3 {}

impl Instruction for DLOAD_3 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }

    fn execute(&mut self, frame: &mut Frame) {
        DLOAD::_dload(frame, 3)
    }
}