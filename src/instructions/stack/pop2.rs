#[allow(non_camel_case_types)]
struct POP2 {}

impl Instruction for POP2 {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        stack.pop_slot();
        stack.pop_slot();
    }
}