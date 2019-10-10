#[allow(non_camel_case_types)]
pub struct FCONST_2 {}

impl Instruction for FCONST_2 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_float(2f32);
    }
}