use crate::rtda::Thread;
use crate::instructions::*;
use std::sync::Arc;
use crate::rtda::heap::Method;
use std::ops::DerefMut;

pub fn interpret(method: Arc<Method>) {
    let thread = Thread::new();
    //let thread = crate::util::arc_util::as_mut_ref(thread);

    let mut t = thread.write().unwrap();
    Thread::new_frame(thread.clone(), method.clone());

    _loop(thread.write().unwrap().deref_mut(), method.code());
}

fn _loop(thread: &mut Thread, bytecode: Arc<Vec<u8>>) {
    let rc_frame = thread.pop_frame();
    let mut frame = rc_frame.borrow_mut();
    let mut reader = BytecodeReader::new(bytecode.as_ref(), 0);

    loop {
        let pc = frame.next_pc();
        thread.set_pc(pc);

        reader.reset(pc);
        let opcode = reader.read_u8();

        let mut inst = new_instruction(opcode);
        inst.fetch_operands(&mut reader);
        frame.set_next_pc(reader.pc());

        // println!("pc:{}, inst:{}", pc, inst);
        inst.execute(frame.deref_mut());

        frame.operand_stack().dbg_print_top();
    }
}