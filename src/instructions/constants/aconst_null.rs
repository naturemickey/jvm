#[allow(non_camel_case_types)]
struct ACONST_NULL {}

impl Instruction for ACONST_NULL {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do.
    }

    fn execute(&mut self,  frame: &mut Frame) {
        frame.operand_stack().push_ref(NULL);
    }
}