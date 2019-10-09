#[allow(non_camel_case_types)]
struct FNEG {}

impl Instruction for FNEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let val = stack.pop_float();
        stack.push_float(-val);
    }
}