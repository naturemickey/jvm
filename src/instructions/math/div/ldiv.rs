#[allow(non_camel_case_types)]
pub struct LDIV {}

impl Instruction for LDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        if v2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        stack.push_long(v1 / v2);
    }
}

impl Debug for LDIV {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}