struct BranchInstruction {
    offset: u32
}

impl BranchInstruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as u32;
    }

    fn branch(&self, frame: &mut Frame) {
        let pc = frame.thread().pc();
        let next_pc = pc + self.offset;
        frame.set_next_pc(next_pc);
    }
}
