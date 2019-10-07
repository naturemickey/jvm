struct BIPUSH {
    val: i8
}

impl Instruction for BIPUSH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_i8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(self.val as i32);
    }
}