#[allow(non_camel_case_types)]
struct LOOKUP_SWITCH {
    default_offset: i32,
    npairs: i32,
    match_offsets: Vec<i32>,
}

impl Instruction for LOOKUP_SWITCH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        reader.skip_padding();
        self.default_offset = reader.read_i32();
        self.npairs = reader.read_i32();
        self.match_offsets = reader.read_i32s(self.npairs * 2);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let key = frame.operand_stack().pop_int();
        let mut i = 0;
        let len = self.npairs * 2;
        while i < len {
            if self.match_offsets[i as usize] == key {
                let offset = self.match_offsets[i as usize + 1];
                BranchInstruction::_branch(frame, offset);
                return;
            }
            i += 2;
        }
        BranchInstruction::_branch(frame, self.default_offset);
    }
}