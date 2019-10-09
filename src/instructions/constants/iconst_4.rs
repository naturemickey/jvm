#[allow(non_camel_case_types)]
struct ICOUNT_4 {}

impl Instruction for ICOUNT_4 {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(4);
    }
}