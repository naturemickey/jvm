pub struct LocalVars {
    max_locals: usize,
    vars: Vec<Slot>,
}

impl LocalVars {
    pub fn new(max_locals: usize) -> LocalVars {
        let mut vars = Vec::with_capacity(max_locals);
        for _ in 0..max_locals {
            vars.push(Slot::Num(0));
        }
        Self { max_locals, vars }
    }

    pub fn set_int(&mut self, index: usize, val: i32) {
        self.vars[index] = Slot::Num(val as u32);
    }

    pub fn get_int(&self, index: usize) -> i32 {
        match self.vars[index] {
            Slot::Num(int) => int as i32,
            _ => panic!("error local var operation for get i32."),
        }
    }

    pub fn set_float(&mut self, index: usize, val: f32) {
        self.vars[index] = Slot::Num(f32::to_bits(val));
    }

    pub fn get_float(&self, index: usize) -> f32 {
        match self.vars[index] {
            Slot::Num(int) => f32::from_bits(int),
            _ => panic!("error local var operation for get f32."),
        }
    }

    pub fn set_long(&mut self, index: usize, val: i64) {
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

    pub fn set_ref(&mut self, index: usize, reference: Object) {
        self.vars[index] = Slot::Ref(reference)
    }

    pub fn get_ref(&self, index: usize) -> &Object {
        match &self.vars[index] {
            Slot::Ref(obj) => obj,
            _ => panic!("error local var operation for get ref."),
        }
    }
}

#[test]
fn test_local_vars() {
    let mut vars = LocalVars::new(100);

    let int1 = 100;
    let int2 = -100;
    let long1 = 2997924580i64;
    let long2 = -2997924580i64;
    let float = 3.1415926f32;
    let double = 2.71828182845f64;

    vars.set_int(0, int1);
    vars.set_int(1, int2);
    vars.set_long(2, long1);
    vars.set_long(4, long2);
    vars.set_float(6, float);
    vars.set_double(7, double);

    let _int1 = vars.get_int(0);
    let _int2 = vars.get_int(1);
    let _long1 = vars.get_long(2);
    let _long2 = vars.get_long(4);
    let _float = vars.get_float(6);
    let _double = vars.get_double(7);

    assert_eq!(int1, _int1);
    assert_eq!(int2, _int2);
    assert_eq!(long1, _long1);
    assert_eq!(long2, _long2);
    assert_eq!(float, _float);
    assert_eq!(double, _double);
}
