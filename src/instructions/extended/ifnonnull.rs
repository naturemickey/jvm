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
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader){
        self.base.fetch_operands(_reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        let obj = frame.operand_stack().pop_ref();
        if obj != NULL {
            self.base.branch(frame);
        }
    }
}