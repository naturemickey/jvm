use std::fmt::{Debug, Formatter, Error};
use crate::rtda::heap::{Object, Slots, Method};
use std::sync::{Arc, RwLock};

pub mod heap;

include!("thread.rs");
include!("stack.rs");
include!("frame.rs");
include!("local_vars.rs");
include!("operand_stack.rs");
include!("slot.rs");