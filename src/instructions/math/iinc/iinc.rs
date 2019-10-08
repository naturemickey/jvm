struct IINC {
    index: usize,
    _const: i32,
}

impl Instruction for IINC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
        self._const = reader.read_i8() as i32;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let local_vars = frame.local_vars();
        let mut val = local_vars.get_int(self.index);
        local_vars.set_int(self.index, val + self._const);
    }
}