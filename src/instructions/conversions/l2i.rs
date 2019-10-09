#[allow(non_camel_case_types)]
struct L2I {}

impl Instruction for L2I {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let l = stack.pop_long();
        let i = l as i32;
        stack.push_int(i);
    }
}