//pub fn borrow_mut<T>(mut arc:&mut Arc<T>) -> &mut T {
//    Arc::get_mut(&mut arc).unwrap()
//}

//use std::sync::Arc;

//pub fn as_mut_ref<'a, T>(ptr: Arc<T>) -> &'a mut T {
//    unsafe { &mut *(Arc::into_raw(ptr) as *mut T) }
//}