#[allow(non_camel_case_types)]
struct FCOUNT_1 {}

impl Instruction for FCOUNT_1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_float(1f32);
    }
}