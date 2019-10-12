pub struct OperandStack {
    max_stack: usize,
    slots: Vec<Slot>,
}

impl OperandStack {
    pub fn new(max_stack: usize) -> OperandStack {
        Self { max_stack, slots: Vec::new() }
    }

    pub fn new128() -> OperandStack {
        Self::new(128)
    }

    pub fn new1k() -> OperandStack {
        Self::new(1024)
    }

    pub fn push_int(&mut self, val: i32) {
        self.push_u32(val as u32)
    }

    pub fn pop_int(&mut self) -> i32 {
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

    pub fn push_float(&mut self, val: f32) {
        if self.slots.len() >= self.max_stack {
            panic!("operand stack over flow in push_float");
        }
        self.slots.push(Slot::Num(f32::to_bits(val)))
    }

    pub fn pop_float(&mut self) -> f32 {
        match self.slots.pop().unwrap() {
            Slot::Num(i) => f32::from_bits(i),
            _ => panic!("error occur in pop_float")
        }
    }

    pub fn push_long(&mut self, val: i64) {
        self.push_u64(val as u64);
    }

    pub fn pop_long(&mut self) -> i64 {
        self.pop_u64() as i64
    }

    fn push_u64(&mut self, val: u64) {
        let low = val as u32;
        let high = (val >> 32) as u32;

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

    pub fn push_double(&mut self, val: f64) {
        let long = f64::to_bits(val);
        self.push_u64(long);
    }

    pub fn pop_double(&mut self) -> f64 {
        let long = self.pop_u64();
        f64::from_bits(long)
    }

    pub fn push_ref(&mut self, obj: Object) {
        if self.slots.len() >= self.max_stack {
            panic!("operand stack over flow in push_ref");
        }
        self.slots.push(Slot::Ref(obj))
    }

    pub fn pop_ref(&mut self) -> Object {
        match self.slots.pop().unwrap() {
            Slot::Ref(obj) => obj,
            _ => panic!("error local var operation for get pop_ref."),
        }
    }

    pub fn push_slot(&mut self, slot: Slot) {
        self.slots.push(slot);
    }

    pub fn pop_slot(&mut self) -> Slot {
        self.slots.pop().unwrap()
    }
    // #[cfg(test)]
    pub fn dbg_print_top(&self) {
        let top = self.slots.last();
        match top {
            Some(x) => Some(dbg!(x)),
            None => None
        };
    }
}

#[test]
fn test_operand_stack() {
    let mut ops = OperandStack::new(100);

    let int1 = 100;
    let int2 = -100;
    let long1 = 2997924580i64;
    let long2 = -2997924580i64;
    let float = 3.1415926f32;
    let double = 2.71828182845f64;

    ops.push_int(int1);
    ops.push_int(int2);
    ops.push_long(long1);
    ops.push_long(long2);
    ops.push_float(float);
    ops.push_double(double);

    let _double = ops.pop_double();
    let _float = ops.pop_float();
    let _long2 = ops.pop_long();
    let _long1 = ops.pop_long();
    let _int2 = ops.pop_int();
    let _int1 = ops.pop_int();

    assert_eq!(int1, _int1);
    assert_eq!(int2, _int2);
    assert_eq!(float, _float);

    println!("{}, {}", long1, _long1);

    assert_eq!(long1, _long1);
    assert_eq!(long2, _long2);
    assert_eq!(double, _double);
}