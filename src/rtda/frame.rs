pub struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
}

impl Frame {
    pub fn new(max_locals: usize, max_stack: usize) -> Box<Frame> {
        let local_vars = LocalVars::new(max_locals);
        let operand_stack = OperandStack::new(max_stack);
        Box::new(Self { local_vars, operand_stack })
    }

    pub fn operand_stack(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }

    pub fn local_vars(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }
}
