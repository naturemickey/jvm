struct Stack {
    deep: usize,
    vec: Vec<Box<Frame>>,
}

impl Stack {
    fn new(deep: usize) -> Stack {
        Self { deep, vec: Vec::with_capacity(deep) }
    }
    fn new1k() -> Stack {
        Self::new(1024)
    }
    fn new1m() -> Stack {
        Self::new(1024 * 1024)
    }

    fn push(&mut self, frame: Box<Frame>) {
        if self.vec.len() >= self.deep {
            panic!("java.lang.StackOverFlowError");
        }
        self.vec.push(frame);
    }
    fn pop(&mut self) -> Box<Frame> {
        match self.vec.pop() {
            Some(f) => f,
            None => panic!("jvm stack is empty!")
        }
    }
    fn top(&self) -> &Box<Frame> {
        match self.vec.last() {
            Some(f) => f,
            None => panic!("jvm stack is empty!")
        }
    }
}