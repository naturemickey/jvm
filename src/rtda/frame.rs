pub struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
    thread: Arc<RwLock<Thread>>,
    method: Arc<Method>,
    next_pc: i32,
}

impl Frame {
    pub fn new(thread: Arc<RwLock<Thread>>,
               method: Arc<Method>) -> Rc<RefCell<Frame>> {
        let local_vars = LocalVars::new(method.max_locals() as usize);
        let operand_stack = OperandStack::new(method.max_stack() as usize);
        Rc::new(RefCell::new(Self { local_vars, operand_stack, thread, method, next_pc: 0 }))
    }

    pub fn operand_stack(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }

    pub fn local_vars(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }

    pub fn thread(&self) -> Arc<RwLock<Thread>> {
        self.thread.clone()
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
        self.thread().read().unwrap().pc
    }

    pub fn method(&self) -> Arc<Method> {
        self.method.clone()
    }
}
