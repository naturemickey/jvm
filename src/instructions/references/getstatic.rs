#[allow(non_camel_case_types)]
pub struct GET_STATIC {
    index: u16
}

impl GET_STATIC {
    fn new(index: u16) -> Self {
        Self { index }
    }
}

impl Instruction for GET_STATIC {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        unimplemented!()
    }
}

impl Debug for GET_STATIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}