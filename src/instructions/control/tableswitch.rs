#[allow(non_camel_case_types)]
pub struct TABLE_SWITCH {
    default_offset: i32,
    low: i32,
    high: i32,
    jump_offsets: Vec<i32>,
}

impl Debug for TABLE_SWITCH {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(default_offset={}, low={}, high={}, jump_offsets={:?})", self.default_offset, self.low, self.high, self.jump_offsets)
    }
}

impl TABLE_SWITCH {
    pub fn new() -> Self {
        Self { default_offset: 0, low: 0, high: 0, jump_offsets: Vec::with_capacity(0) }
    }
}

impl Instruction for TABLE_SWITCH {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        _reader.skip_padding();
        self.default_offset = _reader.read_i32();
        self.low = _reader.read_i32();
        let jump_offsets_count = self.high - self.low + 1;
        self.jump_offsets = _reader.read_i32s(jump_offsets_count);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let index = frame.operand_stack().pop_int();
        let offset = if index >= self.low && index <= self.high {
            self.jump_offsets[(index - self.low) as usize]
        } else {
            self.default_offset
        };
        BranchInstruction::_branch(frame, offset);
    }
}