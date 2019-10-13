use crate::classfile::{ConstantPool, ClassFile};
use std::sync::Arc;
use crate::rtda::Slot;
use std::ops::Index;

include!("object.rs");
include!("class.rs");
include!("classloader.rs");
include!("slots.rs");
include!("access_flags.rs");
include!("field.rs");
include!("method.rs");
include!("classmember.rs");
