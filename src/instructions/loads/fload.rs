struct FLOAD {
    index: u8
}

impl FLOAD {
    fn _fload(frame: &mut Frame, index: usize) {
        let val = frame.local_vars().get_float(index);
        frame.operand_stack().push_float(val);
    }
}

impl Instruction for FLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_fload(frame, self.index as usize);
    }
}