#[allow(non_camel_case_types)]
pub struct NOP {}

impl Instruction for NOP {

    fn execute(&mut self, _frame: &mut Frame) {
        // nothing to do.
    }
}