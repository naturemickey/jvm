pub struct Thread<'a> {
    pc: u32,
    stack: Stack<'a>,
}

impl <'a>Thread<'a> {
    pub fn new() -> Thread <'a>{
        Self { pc: 0, stack: Stack::<'a>::new1k() }
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn push_frame(&'a mut self, frame: Box<Frame<'a>>) {
        self.stack.push(frame);
    }

    pub fn pop_frame(&'a mut self) -> Box<Frame<'a>> {
        self.stack.pop()
    }

    pub fn current_frame(&self) -> &Box<Frame> {
        self.stack.top()
    }
}