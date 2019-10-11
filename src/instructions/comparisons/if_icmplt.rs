#[allow(non_camel_case_types)]
pub struct IF_ICMPLT {
    base: BranchInstruction
}
impl IF_ICMPLT {
    pub fn new() -> Self {
        Self { base: BranchInstruction::new() }
    }
}

impl Instruction for IF_ICMPLT {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.base.fetch_operands(reader);
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let val2 = stack.pop_int();
        let val1 = stack.pop_int();
        if val1 < val2 {
            self.base.branch(frame);
        }
    }
}

impl Debug for IF_ICMPLT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.base.offset)
    }
}