#[allow(non_camel_case_types)]
pub struct INVOKE_VIRTUAL {
    index: u16
}

impl INVOKE_VIRTUAL {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for INVOKE_VIRTUAL {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let class = frame.method().class();
        let class_ref = class.read().unwrap();
        let cp = class_ref.constant_pool();
        let cp_ref = cp.read().unwrap();
        let method_ref = cp_ref.get_constant(self.index).get_method_ref();

        if method_ref.name() == "println" {
            let stack = frame.operand_stack();
            match method_ref.descriptor() {
                "(Z)V" => println!("{}", stack.pop_int() != 0),
                "(C)V" => println!("{}", stack.pop_int()),
                "(I)V" | "(B)V" | "(S)V" => println!("{}", stack.pop_int()),
                "(F)V" => println!("{}", stack.pop_float()),
                "(J)V" => println!("{}", stack.pop_long()),
                "(D)V" => println!("{}", stack.pop_double()),
                desc => panic!("println: {}", desc)
            }
            stack.pop_ref();
        }
    }
}

impl Debug for INVOKE_VIRTUAL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}