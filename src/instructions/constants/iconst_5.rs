#[allow(non_camel_case_types)]
struct ICOUNT_5 {}

impl Instruction for ICOUNT_5 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(5);
    }
}