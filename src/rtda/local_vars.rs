pub struct LocalVars {
    //max_locals: usize,
    slots: Slots,
}

impl LocalVars {
    pub fn new(max_locals: usize) -> LocalVars {
        let slots = Slots::new(max_locals);
        Self { /* max_locals,*/ slots }
    }

    pub fn set_int(&mut self, index: usize, val: i32) {
        self.slots.set_int(index, val)
    }

    pub fn get_int(&self, index: usize) -> i32 {
        self.slots.get_int(index)
    }

    pub fn set_float(&mut self, index: usize, val: f32) {
        self.slots.set_float(index, val)
    }

    pub fn get_float(&self, index: usize) -> f32 {
        self.slots.get_float(index)
    }

    pub fn set_long(&mut self, index: usize, val: i64) {
        self.slots.set_long(index, val)
    }

    pub fn get_long(&self, index: usize) -> i64 {
        self.slots.get_long(index)
    }

    pub fn set_double(&mut self, index: usize, val: f64) {
        self.slots.set_double(index, val)
    }

    pub fn get_double(&self, index: usize) -> f64 {
        self.slots.get_double(index)
    }

    pub fn set_ref(&mut self, index: usize, reference: Rc<Object>) {
        self.slots.set_ref(index, reference)
    }

    pub fn get_ref(&self, index: usize) -> Rc<Object> {
        self.slots.get_ref(index)
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
