#[allow(non_camel_case_types)]
pub struct ISTORE {
    index: usize
}

impl ISTORE {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _istore(frame: &mut Frame, index: usize) {
        let val = frame.operand_stack().pop_int();
        frame.local_vars().set_int(index as usize, val);
    }
}

impl Instruction for ISTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_istore(frame, self.index);
    }
}

impl Debug for ISTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}