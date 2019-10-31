#[allow(non_camel_case_types)]
pub struct DUP_X1 {}

impl Instruction for DUP_X1 {
    /*
    bottom -> top
    [...][c][b][a]
              __/
             |
             V
    [...][c][a][b][a]
    */
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        stack.push_slot(slot1.clone());
        stack.push_slot(slot2);
        stack.push_slot(slot1);
    }
}

impl Debug for DUP_X1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}