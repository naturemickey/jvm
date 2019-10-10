
use crate::classfile::MemberInfo;
use crate::rtda::Thread;
use crate::instructions::{BytecodeReader, new_Instruction};

fn interpret(method_info: &MemberInfo) {
    let code_attr = method_info.code_attribute().unwrap();
    let max_locals = code_attr.max_locals();
    let max_stack = code_attr.max_stack();
    let bytecode = code_attr.code();

    let mut thread = Thread::new();
    let frame = thread.new_frame(max_locals as usize, max_stack as usize);
    thread.push_frame(frame);
}

fn _loop(thread :&mut Thread, bytecode:&Vec<u8>) {
    let mut frame = thread.pop_frame();
    let mut reader = BytecodeReader::new(bytecode, 0);

    loop {
        let pc = frame.next_pc();
        thread.set_pc(pc);

        reader.reset(pc);
        let opcode = reader.read_u8();

        let mut inst = new_Instruction(opcode);
        inst.fetch_operands(&mut reader);
        frame.set_next_pc(reader.pc());
        println!("pc:{}, inst:{}", pc, inst);
        inst.execute(&mut frame);
    }
}