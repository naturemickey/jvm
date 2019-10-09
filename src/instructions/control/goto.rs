#[allow(non_camel_case_types)]
struct GOTO {
    base:BranchInstruction,
}

impl Instruction for GOTO {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        self.base.branch(frame);
    }
}