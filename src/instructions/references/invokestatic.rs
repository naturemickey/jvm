#[allow(non_camel_case_types)]
pub struct INVOKE_STATIC {
    index: u16
}

impl INVOKE_STATIC {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for INVOKE_STATIC {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        // todo for default method and interface static method
        let cp = frame.method().class().read().unwrap().constant_pool();
        let mut cp_ref = cp.write().unwrap();
        let method_ref = cp_ref.get_constant_mut(self.index).get_method_ref_mut();
        let resolved_method = method_ref.resolve_method_ref();
        if !resolved_method.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        // todo for class init

        super::invoke_method(frame, resolved_method);
    }
}

impl Debug for INVOKE_STATIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}