#[allow(non_camel_case_types)]
struct DSTORE {
    index: u8
}

impl DSTORE {
    fn _dstore(frame: &mut Frame, index: usize) {
        let val = frame.operand_stack().pop_double();
        frame.local_vars().set_double(index as usize, val);
    }
}

impl Instruction for DSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_dstore(frame, self.index as usize);
    }
}