#[allow(non_camel_case_types)]
pub struct WIDE {
    modified_instruction: Box<dyn Instruction>
}

impl WIDE {
    pub fn new() -> Self {
        Self { modified_instruction: Box::default() }
    }
}

impl Instruction for WIDE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        let opcode = reader.read_u8();
        let index = reader.read_u16() as usize;
        self.modified_instruction = match opcode {
            0x15 => Box::new(loads::ILOAD::new(index)),
            0x16 => Box::new(loads::LLOAD::new(index)),
            0x17 => Box::new(loads::FLOAD::new(index)),
            0x18 => Box::new(loads::DLOAD::new(index)),
            0x19 => Box::new(loads::ALOAD::new(index)),
            0x36 => Box::new(stores::ISTORE::new(index)),
            0x37 => Box::new(stores::LSTORE::new(index)),
            0x38 => Box::new(stores::FSTORE::new(index)),
            0x39 => Box::new(stores::DSTORE::new(index)),
            0x3a => Box::new(stores::ASTORE::new(index)),
            0x84 => {
                let _const = reader.read_i16() as i32;
                Box::new(math::IINC::new(index, _const))
            }
            0xa9 =>  // ret
                panic!("Unsupported opcode : 0xa9!"),
            opcode =>
                panic!("Unsupported opcode : {}!", opcode),
        };
    }

    fn execute(&mut self, frame: &mut Frame) {
        self.modified_instruction.execute(frame);
    }
}