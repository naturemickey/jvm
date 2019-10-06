struct Thread {
    pc: u32,
    stack: Stack,
}

impl Thread {
    fn new() -> Thread {
        Self { pc: 0, stack: Stack::new1k() }
    }

    fn pc(&self) -> u32 {
        self.pc
    }

    fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    fn push_frame(&mut self, frame: Frame) {
        self.stack.push(frame);
    }

    fn pop_frame(&mut self) -> Frame {
        self.stack.pop()
    }

    fn current_frame(&self) -> &Frame {
        self.stack.top()
    }
}