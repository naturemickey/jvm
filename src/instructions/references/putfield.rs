#[allow(non_camel_case_types)]
pub struct PUT_FIELD {
    index: u16
}

impl PUT_FIELD {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Instruction for PUT_FIELD {
    fn fetch_operands(&mut self, _reader: &mut BytecodeReader) {
        self.index = _reader.read_u16();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_method = frame.method();
        let current_class = current_method.class();
        let cp = current_class.constant_pool();
        let field_ref = unsafe { crate::util::arc_util::borrow_mut(cp.clone()).get_constant_mut(self.index).get_field_ref_mut() };
        let field = field_ref.resolved_field();

        if field.is_static() {
            panic!("java.lang.IncompatibleClassChangError");
        }
        if field.is_final() {
            if current_class.borrow() != field.class().borrow() || current_method.name() != "<init>" {
                panic!("java.lang.IllegalAccessError");
            }
        }

        let descriptor = field.descriptor();
        let slot_id = field.slot_id();
        let stack = frame.operand_stack();

        match descriptor.chars().next() {
            Some(c) => match c {
                'Z' | 'B' | 'C' | 'S' | 'I' => {
                    let val = stack.pop_int();
                    let _ref = stack.pop_ref();
                    if _ref == Object::null() {
                        panic!("java.lang.NullPointerException");
                    }
                    crate::util::arc_util::borrow_mut(_ref.clone()).fields_mut().set_int(slot_id, val);
                }
                'F' => {
                    let val = stack.pop_float();
                    let _ref = stack.pop_ref();
                    if _ref == Object::null() {
                        panic!("java.lang.NullPointerException");
                    }
                    crate::util::arc_util::borrow_mut(_ref.clone()).fields_mut().set_float(slot_id, val);
                }
                'J' => {
                    let val = stack.pop_long();
                    let _ref = stack.pop_ref();
                    if _ref == Object::null() {
                        panic!("java.lang.NullPointerException");
                    }
                    crate::util::arc_util::borrow_mut(_ref.clone()).fields_mut().set_long(slot_id, val);
                }
                'D' => {
                    let val = stack.pop_double();
                    let _ref = stack.pop_ref();
                    if _ref == Object::null() {
                        panic!("java.lang.NullPointerException");
                    }
                    crate::util::arc_util::borrow_mut(_ref.clone()).fields_mut().set_double(slot_id, val);
                }
                'L' | '[' => {
                    let val = stack.pop_ref();
                    let _ref = stack.pop_ref();
                    if _ref == Object::null() {
                        panic!("java.lang.NullPointerException");
                    }
                    crate::util::arc_util::borrow_mut(_ref.clone()).fields_mut().set_ref(slot_id, val);
                }
                _ => panic!("impossible")
            },
            None => panic!("impossible")
        }
    }
}

impl Debug for PUT_FIELD {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}