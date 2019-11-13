#[allow(non_camel_case_types)]
pub struct IINC {
    index: usize,
    _const: i32,
}

impl IINC {
    pub fn new(index: usize, _const: i32) -> Self {
        Self { index: 0, _const: 0 }
    }
    pub fn new00() -> Self {
        Self { index: 0, _const: 0 }
    }
}

impl Instruction for IINC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
        self._const = reader.read_i8() as i32;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let local_vars = frame.local_vars();
        let val = local_vars.get_int(self.index);
        local_vars.set_int(self.index, val + self._const);
    }
}

impl Debug for IINC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(index={}, _const={})", self.index, self._const)
    }
}