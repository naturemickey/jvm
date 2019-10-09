#[allow(non_camel_case_types)]
struct ICOUNT_3 {}

impl Instruction for ICOUNT_3 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(3);
    }
}