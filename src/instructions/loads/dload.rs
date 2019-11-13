#[allow(non_camel_case_types)]
pub struct DLOAD {
    index: usize
}

impl DLOAD {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn new0() -> Self {
        Self { index: 0 }
    }
    fn _dload(frame: &mut Frame, index: usize) {
        let val = frame.local_vars().get_double(index);
        frame.operand_stack().push_double(val);
    }
}

impl Instruction for DLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_dload(frame, self.index);
    }
}

impl Debug for DLOAD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={})", self.index)
    }
}