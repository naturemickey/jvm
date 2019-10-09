#[allow(non_camel_case_types)]
struct DCOUNT_0 {}

impl Instruction for DCOUNT_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_double(0f64);
    }
}