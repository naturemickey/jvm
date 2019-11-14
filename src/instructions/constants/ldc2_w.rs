#[allow(non_camel_case_types)]
pub struct LDC2_W {
    index: u16
}

impl LDC2_W {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for LDC2_W {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.method().class().constant_pool();
        match cp.get_constant(self.index) {
            Constant::Long(l) => frame.operand_stack().push_long(*l),
            Constant::Double(d) => frame.operand_stack().push_double(*d),
            _ => panic!("java.lang.ClassFormatError")
        }
    }
}

impl Debug for LDC2_W {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}