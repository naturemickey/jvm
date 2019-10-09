#[allow(non_camel_case_types)]
struct DCOUNT_1 {}

impl Instruction for DCOUNT_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_double(1f64);
    }
}