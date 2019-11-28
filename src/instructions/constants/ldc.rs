#[allow(non_camel_case_types)]
pub struct LDC {
    index: u8
}

impl LDC {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    fn _ldc(frame: &mut Frame, index: u16) {
        // let stack = frame.operand_stack();
        let cp = frame.method().class().read().unwrap().constant_pool();
        let cp_ref = cp.read().unwrap();

        match cp_ref.get_constant(index as u16) {
            Constant::Integer(i) => frame.operand_stack().push_int(*i),
            Constant::Float(f) => frame.operand_stack().push_float(*f),
//            Constant::String(s) => // todo in chapter8,
//            Constant::Class(c) => // todo in chapter9,
//            Constant::MethodType()
//            Constant::MethodHandle()
            _ => panic!("todo: ldc!")
        }
    }
}

impl Instruction for LDC {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u8();
    }

    fn execute(&mut self, frame: &mut Frame) {
        LDC::_ldc(frame, self.index as u16);
    }
}

impl Debug for LDC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}