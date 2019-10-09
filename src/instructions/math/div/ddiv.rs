#[allow(non_camel_case_types)]
struct DDIV {}

impl Instruction for DDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        stack.push_double(v1 / v2);
    }
}