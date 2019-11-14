#[allow(non_camel_case_types)]
pub struct LDC_W {
    index: u16
}

impl LDC_W {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for LDC_W {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        LDC::_ldc(frame, self.index);
    }
}

impl Debug for LDC_W {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}