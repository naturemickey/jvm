#[allow(non_camel_case_types)]
pub struct ISHR {}

impl Instruction for ISHR {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = frame.operand_stack();
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let s = ((v2 as u32) & 0x1f) as i32;
        stack.push_int(v1 >> s);
    }
}

impl Debug for ISHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "()")
    }
}