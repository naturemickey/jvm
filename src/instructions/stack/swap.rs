#[allow(non_camel_case_types)]
pub struct SWAP {}

impl Instruction for SWAP {
    /*
    bottom -> top
    [...][c][b][a]
              \/
              /\
             V  V
    [...][c][a][b]
    */
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();

        stack.push_slot(slot1);
        stack.push_slot(slot2);
    }
}

impl Debug for SWAP {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}