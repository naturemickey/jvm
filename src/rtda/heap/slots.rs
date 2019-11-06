pub struct Slots {
    slots: Vec<Slot>,
}

impl Slots {
    pub fn new(slot_count: usize) -> Slots {
        let mut slots = Vec::with_capacity(slot_count);
        for _ in 0..slot_count {
            slots.push(Slot::Num(0));
        }
        Self { slots }
    }

    pub fn set_int(&mut self, index: usize, val: i32) {
        self.slots[index] = Slot::Num(val as u32);
    }

    pub fn get_int(&self, index: usize) -> i32 {
        match self.slots[index] {
            Slot::Num(int) => int as i32,
            _ => panic!("error local var operation for get i32."),
        }
    }

    pub fn set_float(&mut self, index: usize, val: f32) {
        self.slots[index] = Slot::Num(f32::to_bits(val));
    }

    pub fn get_float(&self, index: usize) -> f32 {
        match self.slots[index] {
            Slot::Num(int) => f32::from_bits(int),
            _ => panic!("error local var operation for get f32."),
        }
    }

    pub fn set_long(&mut self, index: usize, val: i64) {
        let low = val as u32;
        let high = (val as u64 >> 32) as u32;

        self.slots[index] = Slot::Num(low);
        self.slots[index + 1] = Slot::Num(high);
    }


    fn get_u32(&self, index: usize) -> u32 {
        match self.slots[index] {
            Slot::Num(int) => int,
            _ => panic!("error local var operation for get i32."),
        }
    }

    pub fn get_long(&self, index: usize) -> i64 {
        let low = self.get_u32(index) as u64;
        let high = self.get_u32(index + 1) as u64;
        ((high << 32) | low) as i64
    }

    pub fn set_double(&mut self, index: usize, val: f64) {
        let n = f64::to_bits(val) as i64;
        self.set_long(index, n);
    }

    pub fn get_double(&self, index: usize) -> f64 {
        let n = self.get_long(index) as u64;
        f64::from_bits(n)
    }

    pub fn set_ref(&mut self, index: usize, reference: Arc<Object>) {
        self.slots[index] = Slot::Ref(reference)
    }

    pub fn get_ref(&self, index: usize) -> Arc<Object> {
        match &self.slots[index] {
            Slot::Ref(obj) => obj.clone(),
            _ => panic!("error local var operation for get ref."),
        }
    }
}
