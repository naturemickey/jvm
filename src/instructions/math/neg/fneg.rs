#[allow(non_camel_case_types)]
struct FNEG {}

impl Instruction for FNEG {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let val = stack.pop_float();
        stack.push_float(-val);
    }
}