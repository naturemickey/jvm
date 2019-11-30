#[allow(non_camel_case_types)]
pub struct INSTANCE_OF {
    index: u16
}

impl INSTANCE_OF {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for INSTANCE_OF {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let _ref = stack.pop_ref();
        if _ref.read().unwrap().deref() == Object::null().read().unwrap().deref() {
            stack.push_int(0);
        } else {
            let cp = frame.method().class().read().unwrap().constant_pool();
            let mut cp_ref = cp.write().unwrap();
            let stack = frame.operand_stack();
            let class_ref = cp_ref.get_constant_mut(self.index).get_class_ref_mut();
            let class = class_ref.resolved_class();

            if _ref.read().unwrap().is_instance_of(class.read().unwrap().deref()) {
                stack.push_int(1);
            } else {
                stack.push_int(0);
            }
        }
    }
}

impl Debug for INSTANCE_OF {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}