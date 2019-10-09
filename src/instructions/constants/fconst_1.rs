#[allow(non_camel_case_types)]
struct FCOUNT_1 {}

impl Instruction for FCOUNT_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_float(1f32);
    }
}