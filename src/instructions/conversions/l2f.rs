#[allow(non_camel_case_types)]
struct L2F {}

impl Instruction for L2F {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let l = stack.pop_long();
        let f = l as f32;
        stack.push_float(f);
    }
}