struct OperandStack {
    max_stack: usize,
    slots: Vec<Slot>,
}

impl OperandStack {
    fn new(max_stack: usize) -> OperandStack {
        Self { max_stack, slots: Vec::new() }
    }

    fn new128() -> OperandStack {
        Self::new(128)
    }

    fn new1k() -> OperandStack {
        Self::new(1024)
    }

    fn push_int(&mut self, val: i32) {
        if self.slots.len() >= self.max_stack {
            panic!("operand stack over flow in push_int");
        }
        self.slots.push(Slot::Num(val as u32))
    }

    fn pop_int(&mut self) -> i32 {
        match self.slots.pop().unwrap() {
            Slot::Num(i) => i as i32,
            _ => panic!("error occur in pop_int")
        }
    }

    fn push_float(&mut self, val: f32) {
        if self.slots.len() >= self.max_stack {
            panic!("operand stack over flow in push_float");
        }
        self.slots.push(Slot::Num(f32::to_bits(val)))
    }

    fn pop_float(&mut self) -> f32 {
        match self.slots.pop().unwrap() {
            Slot::Num(i) => f32::from_bits(i),
            _ => panic!("error occur in pop_float")
        }
    }

    fn push_long(&mut self, val: i64) {
        let low = val as u32;
        let high = (val as u64 >> 32) as u32;

        self.push_int(low as i32);
        self.push_int(high as i32);
    }

    fn pop_long(&mut self) -> i64 {
        let hight = self.pop_int() as u64;
        let low = self.pop_int() as u64;
        ((hight << 32) | low) as i64
    }

    fn push_ref(&mut self, obj: Object) {
        if self.slots.len() >= self.max_stack {
            panic!("operand stack over flow in push_ref");
        }
        self.slots.push(Slot::Ref(obj))
    }

    fn pop_ref(&mut self) -> Object {
        match self.slots.pop().unwrap() {
            Slot::Ref(obj) => obj,
            _ => panic!("error local var operation for get pop_ref."),
        }
    }
}