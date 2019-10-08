struct Stack<'a> {
    deep: usize,
    vec: Vec<Box<Frame<'a>>>,
}

impl <'a>Stack <'a>{
    fn new(deep: usize) -> Stack<'a> {
        Self { deep, vec: Vec::with_capacity(deep) }
    }
    fn new1k() -> Stack<'a> {
        Self::new(1024)
    }
    fn new1m() -> Stack<'a> {
        Self::new(1024 * 1024)
    }

    fn push(&'a mut self, frame: Box<Frame<'a>>) {
        if self.vec.len() >= self.deep {
            panic!("java.lang.StackOverFlowError");
        }
        self.vec.push(frame);
    }
    fn pop(&'a mut self) -> Box<Frame<'a>> {
        if self.vec.len() == 0 {
            panic!("jvm stack is empty!");
        }
        self.vec.pop().unwrap()
    }
    fn top(&'a self) -> &Box<Frame<'a>> {
        if self.vec.len() == 0 {
            panic!("jvm stack is empty!");
        }
        &self.vec.last().unwrap()
    }
}