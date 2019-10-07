struct SWAP {}

impl Instruction for SWAP {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }
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