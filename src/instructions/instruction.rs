trait Instruction {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader);
    fn execute(&mut self, frame: &mut Frame);
}