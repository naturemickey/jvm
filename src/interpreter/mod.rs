
use crate::rtda::Thread;
use crate::instructions::*;
use std::sync::Arc;
use std::borrow::Borrow;
use crate::rtda::heap::Method;

pub fn interpret(method: Arc<Method>) {
//    let code_attr = method_info.code_attribute().unwrap();
//    let max_locals = code_attr.max_locals();
//    let max_stack = code_attr.max_stack();
//    let bytecode = dbg!(code_attr.code());

    let mut thread = Thread::new();
    Thread::new_frame(thread.clone(), method.clone());

    let mut thread = crate::util::arc_util::borrow_mut(thread);

    _loop(thread, method.code());
}

fn _loop(thread: &mut Thread, bytecode: Arc<Vec<u8>>) {
    let mut frame = thread.pop_frame();
    let mut reader = BytecodeReader::new(bytecode.borrow(), 0);

    loop {
        let pc = frame.next_pc();
        thread.set_pc(pc);

        reader.reset(pc);
        let opcode = reader.read_u8();

        let mut inst = new_instruction(opcode);
        inst.fetch_operands(&mut reader);
        frame.set_next_pc(reader.pc());

        // println!("pc:{}, inst:{}", pc, inst);
        inst.execute(&mut frame);

        frame.operand_stack().dbg_print_top();
    }
}