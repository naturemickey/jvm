#[allow(non_camel_case_types)]
pub struct PUT_STATIC {
    index: u16
}

impl PUT_STATIC {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for PUT_STATIC {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_method = frame.method();
        let current_class = current_method.class();
        let cp = current_class.write().unwrap().constant_pool();
        let field_ref = unsafe { cp.write().unwrap().get_constant_mut(self.index).get_field_ref_mut() };
        let field = field_ref.resolved_field().read().unwrap();
        let class = field.class();

        if !field.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        if field.is_final() {
            if current_class.read().unwrap().deref() == class.read().unwrap().deref() || current_method.name() != "<clinit>" {
                panic!("java.lang.IllegalAccessError");
            }
        }

        let descriptor = field.descriptor();
        let slot_id = field.slot_id();
        let slots = class.write().unwrap().static_vars_mut();
        let stack = frame.operand_stack();
        match descriptor.chars().next() {
            Some(c) => match c {
                'Z' | 'B' | 'C' | 'S' | 'I' => slots.set_int(slot_id, stack.pop_int()),
                'F' => slots.set_float(slot_id, stack.pop_float()),
                'J' => slots.set_long(slot_id, stack.pop_long()),
                'D' => slots.set_double(slot_id, stack.pop_double()),
                'L' | '[' => slots.set_ref(slot_id, stack.pop_ref()),
                _ => panic!("impossible")
            },
            None => panic!("impossible")
        }
    }
}

impl Debug for PUT_STATIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}