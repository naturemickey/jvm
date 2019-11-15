#[allow(non_camel_case_types)]
pub struct INVOKE_SPECIAL {
    index: u16
}

impl INVOKE_SPECIAL {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for INVOKE_SPECIAL {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().pop_ref();
        // todo
    }
}

impl Debug for INVOKE_SPECIAL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}