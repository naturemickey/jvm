#[allow(non_camel_case_types)]
pub struct NEW {
    index: u16
}

impl NEW {
    fn new(index: u16) -> Self {
        Self { index }
    }
}

impl Instruction for NEW {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.method().class().constant_pool();
        let class_ref = unsafe { cp.get_constant(self.index).get_class_ref_mut() };
        let class = class_ref.resolved_class();
        // todo init class
        unimplemented!()
    }
}

impl Debug for NEW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}