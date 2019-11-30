pub fn invoke_method(invoker_frame: &mut Frame, method: Arc<Method>) {
    let thread = invoker_frame.thread();
    let new_frame = Thread::new_frame(thread.clone(),method.clone());
    let arg_slot_count = method.arg_slot_count();
    if arg_slot_count > 0 {
        let mut i = arg_slot_count - 1;
        while i > 0 {
            let slot = invoker_frame.operand_stack().pop_slot();
            new_frame.borrow_mut().local_vars().set_slot(i, slot);
            i -= 1;
        }
    }

    // hack!
    if method.is_native() {
        if method.name().eq("registerNatives") {
            thread.write().unwrap().pop_frame();
        } else {
            panic!("native method: {}.{}{}", method.class().read().unwrap().name(), method.name(), method.descriptor());
        }
    }
}