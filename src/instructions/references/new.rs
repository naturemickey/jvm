#[allow(non_camel_case_types)]
pub struct NEW {
    index: u16
}

impl NEW {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for NEW {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.method().class().constant_pool();
        let class_ref = unsafe { crate::util::arc_util::borrow_mut(cp).get_constant_mut(self.index).get_class_ref_mut() };
        let class = class_ref.resolved_class();

        if class.is_interface() || class.is_abstract() {
            panic!("java.lang.InstantiationError")
        }

        let reference = Class::new_object(class.clone());
        frame.operand_stack().push_ref(reference.clone());
    }
}

impl Debug for NEW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}