#[allow(non_camel_case_types)]
pub struct GOTO {
    base:BranchInstruction,
}

impl GOTO {
    pub fn new() -> Self {
        Self { base: BranchInstruction::new() }
    }
}
impl Instruction for GOTO {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        self.base.branch(frame);
    }
}

impl Debug for GOTO {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.base.offset)
    }
}