#[allow(non_camel_case_types)]
pub struct IFLT {
    base: BranchInstruction,
}

impl IFLT {
    pub fn new() -> Self {
        Self { base: BranchInstruction::new() }
    }
}
impl Instruction for IFLT {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let val = frame.operand_stack().pop_int();
        if val < 0 {
            self.base.branch(frame);
        }
    }
}

impl Debug for IFLT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.base.offset)
    }
}