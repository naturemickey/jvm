#[allow(non_camel_case_types)]
struct F2D {}

impl Instruction for F2D {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let f = stack.pop_float();
        let d = f as f64;
        stack.push_double(d);
    }
}