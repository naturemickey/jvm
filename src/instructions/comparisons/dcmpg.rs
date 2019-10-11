#[allow(non_camel_case_types)]
pub struct DCMPG {}

impl Instruction for DCMPG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        stack.push_int(
            if v1 > v2 {
                1
            } else if v1 == v2 {
                0
            } else if v1 < v2 {
                -1
            } else {
                1
            });
    }
}

impl Debug for DCMPG {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}