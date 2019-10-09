#[allow(non_camel_case_types)]
struct L2D {}

impl Instruction for L2D {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let l = stack.pop_long();
        let d = l as f64;
        stack.push_double(d);
    }
}