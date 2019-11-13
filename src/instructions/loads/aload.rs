#[allow(non_camel_case_types)]
pub struct ALOAD {
    index: usize
}

impl ALOAD {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _aload(frame: &mut Frame, index: usize) {
        let val = frame.local_vars().get_ref(index);
        frame.operand_stack().push_ref(val);
    }
}

impl Instruction for ALOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_aload(frame, self.index);
    }
}

impl Debug for ALOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}