#[allow(non_camel_case_types)]
struct FCOUNT_0 {}

impl Instruction for FCOUNT_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_float(0f32);
    }
}