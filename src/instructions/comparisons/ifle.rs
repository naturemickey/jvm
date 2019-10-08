struct Ifle {
    base: BranchInstruction,
}

impl Instruction for Ifle {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        if val <= 0 {
            self.base.branch(frame);
        }
    }
}