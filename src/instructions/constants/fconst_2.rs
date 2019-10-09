#[allow(non_camel_case_types)]
struct FCOUNT_2 {}

impl Instruction for FCOUNT_2 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_float(2f32);
    }
}