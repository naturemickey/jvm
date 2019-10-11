#[allow(non_camel_case_types)]
pub struct IREM {}

impl Instruction for IREM {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        if v2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        stack.push_int(v1 % v2);
    }
}

impl Debug for IREM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}