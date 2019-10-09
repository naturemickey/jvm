#[allow(non_camel_case_types)]
struct IXOR {}

impl Instruction for IXOR {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        stack.push_int(v1 ^ v2);
    }
}