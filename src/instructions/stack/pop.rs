#[allow(non_camel_case_types)]
pub struct POP {}

impl Instruction for POP {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        stack.pop_slot();
    }
}