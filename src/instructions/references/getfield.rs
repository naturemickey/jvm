#[allow(non_camel_case_types)]
pub struct GET_FIELD {
    index: u16
}

impl GET_FIELD {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for GET_FIELD {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.method().class().read().unwrap().constant_pool();
        let mut cp_ref = cp.write().unwrap();
        let field_ref = cp_ref.get_constant_mut(self.index).get_field_ref_mut().resolved_field();
        let field = field_ref.read().unwrap();

        if field.is_static() {
            panic!("java.lang.IncompatibleClassChangError");
        }

        let stack = frame.operand_stack();
        let mut _ref = stack.pop_ref();
        if _ref.read().unwrap().deref() == Object::null().read().unwrap().deref() {
            panic!("java.lang.NullPointerException");
        }

        let descriptor = field.descriptor();
        let slot_id = field.slot_id();
        let mut ref_write = _ref.write().unwrap();
        let slots = ref_write.fields_mut();

        match descriptor.chars().next() {
            Some(c) => match c {
                'Z' | 'B' | 'C' | 'S' | 'I' => stack.push_int(slots.get_int(slot_id)),
                'F' => stack.push_float(slots.get_float(slot_id)),
                'J' => stack.push_long(slots.get_long(slot_id)),
                'D' => stack.push_double(slots.get_double(slot_id)),
                'L' | '[' => stack.push_ref(slots.get_ref(slot_id)),
                _ => panic!("impossible")
            },
            None => panic!("impossible")
        }
    }
}

impl Debug for GET_FIELD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}