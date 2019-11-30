struct Stack {
    deep: usize,
    vec: Vec<Rc<RefCell<Frame>>>,
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

    fn push(&mut self, frame: Rc<RefCell<Frame>>) {
        if self.vec.len() >= self.deep {
            panic!("java.lang.StackOverFlowError");
        }
        self.vec.push(frame);
    }
    fn pop(&mut self) -> Rc<RefCell<Frame>> {
        match self.vec.pop() {
            Some(f) => f,
            None => panic!("jvm stack is empty!")
        }
    }
    fn top(&self) -> Rc<RefCell<Frame>> {
        match self.vec.last() {
            Some(f) => f.clone(),
            None => panic!("jvm stack is empty!")
        }
    }
}