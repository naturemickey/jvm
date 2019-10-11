#[allow(non_camel_case_types)]
pub struct LSHR {}

impl Instruction for LSHR {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_long();
        let s = ((v2 as u32) & 0x3f) as i64;
        stack.push_long(v1 >> s);
    }
}

impl Debug for LSHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}