#[allow(non_camel_case_types)]
pub struct IF_ACMPEQ {
    base: BranchInstruction
}

impl IF_ACMPEQ {
    pub fn new() -> Self {
        Self { base: BranchInstruction::new() }
    }
}

impl Instruction for IF_ACMPEQ {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let val2 = stack.pop_ref().read().unwrap();
        let val1 = stack.pop_ref().read().unwrap();
        if val1.deref() == val2.deref() {
            self.base.branch(frame);
        }
    }
}

impl Debug for IF_ACMPEQ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.base.offset)
    }
}