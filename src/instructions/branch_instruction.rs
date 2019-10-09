struct BranchInstruction {
    offset: i32
}

impl BranchInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32;
    }

    fn branch(&self, frame: &mut Frame) {
        Self::_branch(frame, self.offset);
    }

    fn _branch(frame: &mut Frame, offset: i32) {
        let pc = frame.thread().pc();
        let next_pc = pc + offset;
        frame.set_next_pc(next_pc);
    }
}
