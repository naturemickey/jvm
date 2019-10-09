#[allow(non_camel_case_types)]
struct ICOUNT_0 {}

impl Instruction for ICOUNT_0 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(0);
    }
}