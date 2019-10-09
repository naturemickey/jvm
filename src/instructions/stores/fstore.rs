#[allow(non_camel_case_types)]
struct FSTORE {
    index: u8
}

impl FSTORE {
    fn _fstore(frame: &mut Frame, index: usize) {
        let val = frame.operand_stack().pop_float();
        frame.local_vars().set_float(index as usize, val);
    }
}

impl Instruction for FSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_fstore(frame, self.index as usize);
    }
}