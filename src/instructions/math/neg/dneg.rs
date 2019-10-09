#[allow(non_camel_case_types)]
struct DNEG {}

impl Instruction for DNEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let val = stack.pop_double();
        stack.push_double(-val);
    }
}