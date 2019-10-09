trait Instruction {

    fn fetch_operands(&mut self, _reader: &mut BytecodeReader){
        // nothing to do
    }
    fn execute(&mut self, frame: &mut Frame);
}