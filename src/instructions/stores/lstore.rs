#[allow(non_camel_case_types)]
struct LSTORE {
    index: u8
}

impl LSTORE {
    fn _lstore(frame: &mut Frame, index: usize) {
        let val = frame.operand_stack().pop_long();
        frame.local_vars().set_long(index as usize, val);
    }
}

impl Instruction for LSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_lstore(frame, self.index as usize);
    }
}