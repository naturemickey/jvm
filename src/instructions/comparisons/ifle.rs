#[allow(non_camel_case_types)]
pub struct IFLE {
    base: BranchInstruction,
}
impl IFLE {
    pub fn new() -> Self {
        Self { base: BranchInstruction::new() }
    }
}

impl Instruction for IFLE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        if val <= 0 {
            self.base.branch(frame);
        }
    }
}

impl Debug for IFLE {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.base.offset)
    }
}