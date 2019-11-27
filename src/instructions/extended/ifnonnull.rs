#[allow(non_camel_case_types)]
pub struct IFNONNULL {
    base: BranchInstruction
}

impl IFNONNULL {
    pub fn new() -> Self {
        Self { base: BranchInstruction::new() }
    }
}

impl Instruction for IFNONNULL {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.base.fetch_operands(_reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        let obj = frame.operand_stack().pop_ref();
        if obj.read().unwrap().deref() != Object::null().read().unwrap().deref() {
            self.base.branch(frame);
        }
    }
}

impl Debug for IFNONNULL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(offset={})", self.base.offset)
    }
}