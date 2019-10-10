pub trait Instruction {

    fn fetch_operands(&mut self, _reader: &mut BytecodeReader){
        // nothing to do
    }
    fn execute(&mut self, frame: &mut Frame);
}

enum Inst {

}

pub fn new_Instruction(opcode:u8) -> Instruction {
    unimplemented!()
}