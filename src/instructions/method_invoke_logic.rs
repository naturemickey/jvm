pub fn invoke_method(invoker_frame: &mut Frame, method: &Method) {
    let thread = invoker_frame.thread_mut();
    let new_frame = Thread::new_frame(thread, method);
    let arg_slot_count = method.arg_slot_count();
    if arg_slot_count > 0 {
        let mut i = arg_slot_count - 1;
        while i > 0 {
            let slot = invoker_frame.operand_stack().pop_slot();
            new_frame.as_mut().local_vars().set_slot(i, slot);
            i -= 1;
        }
    }
    unimplemented!()
}