struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
}

impl Frame {
    fn new(max_locals: usize, max_stack: usize) -> Box<Frame> {
        let local_vars = LocalVars::new(max_locals);
        let operand_stack = OperandStack::new(max_stack);
        Box::new(Self { local_vars, operand_stack })
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
    vars.set_long(4,long2);
    vars.set_float(6,float);
    vars.set_double(7,double);

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