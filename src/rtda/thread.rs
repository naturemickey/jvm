pub struct Thread {
    pc: i32,
    stack: Stack,
}

impl Thread {
    pub fn new() -> Arc<RwLock<Thread>> {
        Arc::new(RwLock::new(Self { pc: 0, stack: Stack::new1k() }))
    }

    pub fn pc(&self) -> i32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: i32) {
        self.pc = pc;
    }

    pub fn push_frame(&mut self, frame: Box<Frame>) {
        self.stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Box<Frame> {
        self.stack.pop()
    }

    pub fn current_frame(&self) -> &Box<Frame> {
        self.stack.top()
    }

    pub fn new_frame(&mut self, method: &Method) -> &Box<Frame> {
        self.push_frame(Frame::new(self, method));
        self.stack.top()
    }
}