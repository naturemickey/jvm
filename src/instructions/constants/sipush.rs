#[allow(non_camel_case_types)]
struct SIPUSH {
    val: i16
}

impl Instruction for SIPUSH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.val = reader.read_i16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        frame.operand_stack().push_int(self.val as i32);
    }
}