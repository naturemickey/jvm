#[allow(non_camel_case_types)]
pub struct LLOAD {
    index: usize
}

impl LLOAD {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _lload(frame: &mut Frame, index: usize) {
        let val = frame.local_vars().get_long(index);
        frame.operand_stack().push_long(val);
    }
}

impl Instruction for LLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_lload(frame, self.index);
    }
}

impl Debug for LLOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}