struct ISTORE {
    index: u8
}

impl ISTORE {
    fn _istore(frame: &mut Frame, index: usize) {
        let val = frame.operand_stack().pop_int();
        frame.local_vars().set_int(index as usize, val);
    }
}

impl Instruction for ISTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_istore(frame, self.index as usize);
    }
}