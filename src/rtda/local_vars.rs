struct LocalVars {
    max_locals: usize,
    vars: Vec<Slot>,
}

impl LocalVars {
    fn new(max_locals: usize) -> LocalVars {
        let mut vars = Vec::with_capacity(max_locals);
        for _ in 0..max_locals {
            vars.push(Slot::Num(0));
        }
        Self { max_locals, vars }
    }

    fn set_int(&mut self, index: usize, val: i32) {
        self.vars[index] = Slot::Num(val as u32);
    }

    fn get_int(&self, index: usize) -> i32 {
        match self.vars[index] {
            Slot::Num(int) => int as i32,
            _ => panic!("error local var operation for get i32."),
        }
    }

    fn set_float(&mut self, index: usize, val: f32) {
        self.vars[index] = Slot::Num(f32::to_bits(val));
    }

    fn get_float(&self, index: usize) -> f32 {
        match self.vars[index] {
            Slot::Num(int) => f32::from_bits(int),
            _ => panic!("error local var operation for get f32."),
        }
    }

    fn set_long(&mut self, index: usize, val: i64) {
        let low = val as u32;
        let high = (val as u64 >> 32) as u32;

        self.vars[index] = Slot::Num(low);
        self.vars[index + 1] = Slot::Num(high);
    }


    fn get_u32(&self, index: usize) -> u32 {
        match self.vars[index] {
            Slot::Num(int) => int,
            _ => panic!("error local var operation for get i32."),
        }
    }

    fn get_long(&self, index: usize) -> i64 {
        let low = self.get_u32(index) as u64;
        let high = self.get_u32(index + 1) as u64;
        ((high << 32) | low) as i64
    }

    fn set_double(&mut self, index: usize, val: f64) {
        let n = f64::to_bits(val) as i64;
        self.set_long(index, n);
    }

    fn get_double(&self, index: usize) -> f64 {
        let n = self.get_long(index) as u64;
        f64::from_bits(n)
    }

    fn set_ref(&mut self, index: usize, reference: Object) {
        self.vars[index] = Slot::Ref(reference)
    }

    fn get_ref(&self, index: usize) -> &Object {
        match &self.vars[index] {
            Slot::Ref(obj) => obj,
            _ => panic!("error local var operation for get ref."),
        }
    }
}
