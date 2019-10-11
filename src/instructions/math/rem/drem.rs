#[allow(non_camel_case_types)]
pub struct DREM {}

impl Instruction for DREM {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        let result = v1.rem_euclid(v2);
        stack.push_double(result);
    }
}

impl Debug for DREM {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}