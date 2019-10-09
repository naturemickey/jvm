#[allow(non_camel_case_types)]
struct ICOUNT_2 {}

impl Instruction for ICOUNT_2 {
    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(2);
    }
}