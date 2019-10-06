struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
}

impl Frame {
    fn new(max_locals: usize, max_stack: usize) -> Box<Frame> {
        let local_vars = LocalVars::new(max_locals);
        let operand_stack = OperandStack::new(max_stack);
        Box::new(Self { local_vars, operand_stack })
    }
}