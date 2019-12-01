#[allow(non_camel_case_types)]
pub struct A_RETURN {}

impl Instruction for A_RETURN{
    fn execute(&mut self, frame: &mut Frame) {
        let t = frame.thread();
        let mut thread = t.write().unwrap();
        let current_frame = thread.pop_frame();
        let invoker_frame = thread.top_frame();
        let ret_val = current_frame.borrow_mut().operand_stack().pop_ref();
        invoker_frame.borrow_mut().operand_stack().push_ref(ret_val);
    }
}

impl Debug for A_RETURN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),  Error> {
        write!(f, "()")
    }
}