#[allow(non_camel_case_types)]
struct ALOAD {
    index: u8
}

impl ALOAD {
    fn _aload(frame: &mut Frame, index: usize) {
        let val = frame.local_vars().get_ref(index);
        frame.operand_stack().push_ref(val);
    }
}

impl Instruction for ALOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_aload(frame, self.index as usize);
    }
}