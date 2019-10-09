#[allow(non_camel_case_types)]
struct NOP {}

impl Instruction for NOP {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        // nothing to do.
    }
}