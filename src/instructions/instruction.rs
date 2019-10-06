trait Instruction {
    fn fetch_operands(reader: &BytecodeReader);
    fn execute(frame: &Frame);
}