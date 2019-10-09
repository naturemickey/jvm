#[allow(non_camel_case_types)]
struct LCOUNT_0 {}

impl Instruction for LCOUNT_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_long(0);
    }
}