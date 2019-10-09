#[allow(non_camel_case_types)]
struct FREM {}

impl Instruction for FREM {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        let result = v1.rem_euclid(v2);
        stack.push_float(result);
    }
}