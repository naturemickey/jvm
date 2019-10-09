#[allow(non_camel_case_types)]
struct INEG {}

impl Instruction for INEG {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let val = stack.pop_int();
        stack.push_int(-val);
    }
}