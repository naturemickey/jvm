struct ASTORE {
    index: u8
}

impl ASTORE {
    fn _astore(frame: &mut Frame, index: usize) {
        let val = frame.operand_stack().pop_ref();
        frame.local_vars().set_ref(index as usize, val);
    }
}

impl Instruction for ASTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_astore(frame, self.index as usize);
    }
}