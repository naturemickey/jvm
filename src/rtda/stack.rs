struct Stack {}

impl Stack {
    fn new(deep: u16) -> Stack {
        unimplemented!()
    }
    fn new1k() -> Stack {
        Self::new(1024)
    }
    fn new1m() -> Stack {
        Self::new(1024 * 1024)
    }

    fn push(&mut self, frame:Frame) {
        unimplemented!()
    }
    fn pop(&mut self) -> Frame {
        unimplemented!()
    }
    fn top(&self) -> &Frame {
        unimplemented!()
    }
}