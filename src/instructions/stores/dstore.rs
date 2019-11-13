#[allow(non_camel_case_types)]
pub struct DSTORE {
    index: usize
}

impl DSTORE {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _dstore(frame: &mut Frame, index: usize) {
        let val = frame.operand_stack().pop_double();
        frame.local_vars().set_double(index as usize, val);
    }
}

impl Instruction for DSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_dstore(frame, self.index);
    }
}

impl Debug for DSTORE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}