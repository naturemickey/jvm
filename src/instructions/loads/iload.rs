#[allow(non_camel_case_types)]
struct ILOAD {
    index: u8
}

impl ILOAD {
    fn _iload(frame: &mut Frame, index: usize) {
        let val = frame.local_vars().get_int(index);
        frame.operand_stack().push_int(val);
    }
}

impl Instruction for ILOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_iload(frame, self.index as usize);
    }
}