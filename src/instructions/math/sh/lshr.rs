#[allow(non_camel_case_types)]
struct LSHR {}

impl Instruction for LSHR {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_long();
        let s = ((v2 as u32) & 0x3f) as i64;
        stack.push_long(v1 >> s);
    }
}