#[allow(non_camel_case_types)]
pub struct GET_STATIC {
    index: u16
}

impl GET_STATIC {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for GET_STATIC {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let cp = frame.method().class().read().unwrap().constant_pool();
        let mut cp_ref = cp.write().unwrap();
        let field_ref = unsafe { cp_ref.get_constant_mut(self.index).get_field_ref_mut() }.resolved_field();
        let field = field_ref.read().unwrap();
        let class = field.class();

        if !field.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let descriptor = field.descriptor();
        let slot_id = field.slot_id();
        let mut class_ref = class.write().unwrap();
        let slots = class_ref.static_vars_mut();
        let stack = frame.operand_stack();
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

impl Debug for GET_STATIC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}