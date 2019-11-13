#[allow(non_camel_case_types)]
pub struct ILOAD {
    index: usize
}

impl ILOAD {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _iload(frame: &mut Frame, index: usize) {
        let val = frame.local_vars().get_int(index);
        frame.operand_stack().push_int(val);
    }
}

impl Instruction for ILOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_iload(frame, self.index);
    }
}

impl Debug for ILOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}