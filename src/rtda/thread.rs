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

    pub fn push_frame(&mut self, frame: Rc<RefCell<Frame>>) {
        self.stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Rc<RefCell<Frame>> {
        self.stack.pop()
    }

    pub fn top_frame(&self) -> Rc<RefCell<Frame>> {
        self.stack.top()
    }

    pub fn current_frame(&self) -> Rc<RefCell<Frame>> {
        self.stack.top()
    }

    pub fn new_frame(thread: Arc<RwLock<Thread>>, method: Arc<Method>) -> Rc<RefCell<Frame>> {
        thread.write().unwrap().push_frame(Frame::new(thread.clone(), method));
        thread.read().unwrap().stack.top()
    }
}