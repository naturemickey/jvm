#[allow(non_camel_case_types)]
pub struct LXOR {}

impl Instruction for LXOR {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        stack.push_long(v1 ^ v2);
    }
}