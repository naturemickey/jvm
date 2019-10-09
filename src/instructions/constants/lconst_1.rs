#[allow(non_camel_case_types)]
struct LCOUNT_1 {}

impl Instruction for LCOUNT_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_long(1);
    }
}