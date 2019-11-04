use std::fmt::{Debug, Formatter};
use std::fmt::Error;
use crate::instructions::{Instruction, BytecodeReader};
use crate::rtda::Frame;
use crate::rtda::heap::Constant;
include!("anewarray.rs");
include!("arraylength.rs");
include!("athrow.rs");
include!("checkcast.rs");
include!("getfield.rs");
include!("getstatic.rs");
include!("instanceof.rs");
include!("invokedynamic.rs");
include!("invokeinterface.rs");
include!("invokespecial.rs");
include!("invokestatic.rs");
include!("invokevirtual.rs");
include!("monitorenter.rs");
include!("monitorexit.rs");
include!("new.rs");
include!("newarray.rs");
include!("putfield.rs");
include!("putstatic.rs");
