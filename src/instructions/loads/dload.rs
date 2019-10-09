#[allow(non_camel_case_types)]
struct DLOAD {
    index: u8
}

impl DLOAD {
    fn _dload(frame: &mut Frame, index: usize) {
        let val = frame.local_vars().get_double(index);
        frame.operand_stack().push_double(val);
    }
}

impl Instruction for DLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        Self::_dload(frame, self.index as usize);
    }
}