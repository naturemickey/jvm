#[allow(non_camel_case_types)]
struct ICOUNT_M1 {}

impl Instruction for ICOUNT_M1 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(-1);
    }
}