#[allow(non_camel_case_types)]
struct IF_ICMPGE {
    base: BranchInstruction
}

impl Instruction for IF_ICMPGE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let val2 = stack.pop_int();
        let val1 = stack.pop_int();
        if val1 >= val2 {
            self.base.branch(frame);
        }
    }
}