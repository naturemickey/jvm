#[allow(non_camel_case_types)]
struct IFNULL {
    base: BranchInstruction
}

impl Instruction for IFNULL {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader){
        self.base.fetch_operands(_reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        let obj = frame.operand_stack().pop_ref();
        if obj == NULL {
            self.base.branch(frame);
        }
    }
}