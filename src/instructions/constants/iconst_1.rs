#[allow(non_camel_case_types)]
struct ICOUNT_1 {}

impl Instruction for ICOUNT_1 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(1);
    }
}