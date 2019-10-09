#[allow(non_camel_case_types)]
struct D2F {}

impl Instruction for D2F {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let d = stack.pop_double();
        let f = d as f32;
        stack.push_float(f);
    }
}