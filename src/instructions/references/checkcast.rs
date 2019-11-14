#[allow(non_camel_case_types)]
pub struct CHECK_CAST {
    index: u16
}

impl CHECK_CAST {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for CHECK_CAST {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let _ref = stack.pop_ref();
        if _ref != Object::null() {
            let cp = frame.method().class().constant_pool();
            let class_ref = unsafe { crate::util::arc_util::borrow_mut(cp.clone()).get_constant_mut(self.index).get_class_ref_mut() };
            let class = class_ref.resolved_class();

            if !_ref.is_instance_of(class.borrow()) {
                panic!("java.lang.ClassCastException")
            }
        }
    }
}

impl Debug for CHECK_CAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}