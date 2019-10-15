use crate::classfile::{ConstantPool, ClassFile, MemberInfo};
use std::sync::Arc;
use std::borrow::{Borrow, BorrowMut};

include!("object.rs");
include!("class.rs");
include!("classloader.rs");
include!("slots.rs");
include!("access_flags.rs");
include!("field.rs");
include!("method.rs");
include!("classmember.rs");
