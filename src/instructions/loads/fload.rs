#[allow(non_camel_case_types)]
pub struct FLOAD {
    index: usize
}

impl FLOAD {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    fn _fload(frame: &mut Frame, index: usize) {
        let val = frame.local_vars().get_float(index);
        frame.operand_stack().push_float(val);
    }
}

impl Instruction for FLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_fload(frame, self.index);
    }
}