pub struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
    thread: *const Thread,
    next_pc: i32,
}

impl Frame {
    pub fn new(max_locals: usize, max_stack: usize, thread: *const Thread) -> Frame {
        let local_vars = LocalVars::new(max_locals);
        let operand_stack = OperandStack::new(max_stack);
        Self { local_vars, operand_stack, thread, next_pc: 0 }
    }

    pub fn operand_stack(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }

    pub fn local_vars(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }

    pub fn thread(&self) -> *const Thread {
        self.thread
    }

    pub fn next_pc(&self) -> i32 {
        self.next_pc
    }

    pub fn set_next_pc(&mut self, next_pc: i32) {
        self.next_pc = next_pc;
    }

    pub fn branch(&mut self, offset: i32) {
        self.set_next_pc(self.thread_pc() + offset);
    }

    pub fn thread_pc(&self) -> i32 {
        unsafe {
            (*self.thread).pc
        }
    }
}
