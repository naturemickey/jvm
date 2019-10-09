#[allow(non_camel_case_types)]
struct GOTO_W {
    offset: i32
}

impl Instruction for GOTO_W {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.offset = _reader.read_i32();
    }

    fn execute(&mut self, frame: &mut Frame) {
        BranchInstruction::_branch(frame, self.offset);
    }
}