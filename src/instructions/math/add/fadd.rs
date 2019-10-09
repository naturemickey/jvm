#[allow(non_camel_case_types)]
struct FADD {}

impl Instruction for FADD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v1 = stack.pop_float();
        let v2 = stack.pop_float();
        stack.push_float(v1 + v2);
    }
}