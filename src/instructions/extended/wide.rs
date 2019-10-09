#[allow(non_camel_case_types)]
struct WIDE {
    modified_instruction: Instruction
}

impl Instruction for WIDE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        match reader.read_u8() {
            0x15 => {
                let inst = loads::ILOAD::new();
            }
            0x16 => {}
            0x17 => {}
            0x18 => {}
            0x19 => {}
            0x36 => {}
            0x37 => {}
            0x38 => {}
            0x39 => {}
            0x3a => {}
            0x84 => {}
            0xa9 => {
                panic!("Unsupported opcode : 0xa9!");
            }
            opcode => {
                panic!("Unsupported opcode : {}!", opcode);
            }
        }
    }

    fn execute(&mut self, frame: &mut Frame) {
        self.modified_instruction.execute(frame);
    }
}