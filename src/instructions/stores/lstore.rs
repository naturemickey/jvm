#[allow(non_camel_case_types)]
pub struct LSTORE {
    index: usize
}

impl LSTORE {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _lstore(frame: &mut Frame, index: usize) {
        let val = frame.operand_stack().pop_long();
        frame.local_vars().set_long(index as usize, val);
    }
}

impl Instruction for LSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_lstore(frame, self.index);
    }
}

impl Debug for LSTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}