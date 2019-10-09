#[allow(non_camel_case_types)]
struct LSHL {}

impl Instruction for LSHL {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_long();
        let s = ((v2 as u32) & 0x3f) as i64;
        stack.push_long(v1 << s);
    }
}