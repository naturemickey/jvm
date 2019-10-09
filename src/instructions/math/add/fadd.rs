#[allow(non_camel_case_types)]
struct FADD {}

impl Instruction for FADD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v1 = stack.pop_float();
        let v2 = stack.pop_float();
        stack.push_float(v1 + v2);
    }
}