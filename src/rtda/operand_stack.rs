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
        self.push_u32(val as u32)
    }

    fn pop_int(&mut self) -> i32 {
        self.pop_u32() as i32
    }
    fn push_u32(&mut self, val: u32) {
        if self.slots.len() >= self.max_stack {
            panic!("operand stack over flow in push_u32");
        }
        self.slots.push(Slot::Num(val))
    }

    fn pop_u32(&mut self) -> u32 {
        match self.slots.pop().unwrap() {
            Slot::Num(i) => i,
            _ => panic!("error occur in pop_u32")
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
        self.push_u64(val as u64);
    }

    fn pop_long(&mut self) -> i64 {
        self.pop_u64() as i64
    }

    fn push_u64(&mut self, val:u64) {
        let low = val as u32;
        let high = (val>> 32) as u32;

        println!("push long:{}, low:{}, high:{}", val, low, high);

        self.push_u32(low);
        self.push_u32(high);
    }

    fn pop_u64(&mut self) -> u64 {
        let high = self.pop_u32() as u64;
        let low = self.pop_u32() as u64;
        println!("pop long:{}, low:{}, high:{}", ((high << 32) | low), low, high);
        (high << 32) | low
    }

    fn push_double(&mut self, val: f64) {
        let long = f64::to_bits(val);
        self.push_u64(long);
    }

    fn pop_double(&mut self) -> f64 {
        let long = self.pop_u64();
        f64::from_bits(long)
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