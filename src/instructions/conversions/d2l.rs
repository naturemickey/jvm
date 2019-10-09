#[allow(non_camel_case_types)]
struct D2L {}

impl Instruction for D2L {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let d = stack.pop_double();
        let l = d as i64;
        stack.push_long(l);
    }
}