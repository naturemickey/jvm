#[allow(non_camel_case_types)]
struct DCOUNT_0 {}

impl Instruction for DCOUNT_0 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_double(0f64);
    }
}