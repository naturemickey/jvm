pub struct Frame<'a> {
    local_vars: LocalVars,
    operand_stack: OperandStack,
    thread: &'a Thread<'a>,
    next_pc: i32,
}

impl<'a> Frame<'a> {
    pub fn new(max_locals: usize, max_stack: usize, thread: &'a Thread) -> Box<Frame<'a>> {
        let local_vars = LocalVars::new(max_locals);
        let operand_stack = OperandStack::new(max_stack);
        Box::new(Self { local_vars, operand_stack, thread, next_pc: 0 })
    }

    pub fn operand_stack(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }

    pub fn local_vars(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }

    pub fn thread(&'a self) -> &'a Thread {
        self.thread
    }

    pub fn next_pc(&self) -> i32 {
        self.next_pc
    }

    pub fn set_next_pc(&mut self, next_pc: i32) {
        self.next_pc = next_pc;
    }
}
