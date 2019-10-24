use crate::classfile;
use crate::classfile::{ClassFile, MemberInfo, ConstantInfo};
use std::sync::Arc;
use std::borrow::Borrow;

include!("object.rs");
include!("class.rs");
include!("classloader.rs");
include!("slots.rs");
include!("access_flags.rs");
include!("field.rs");
include!("method.rs");
include!("classmember.rs");

include!("cp/constant.rs");
include!("cp/classref.rs");
include!("cp/constant_pool.rs");
include!("cp/fieldref.rs");
include!("cp/interface_methodref.rs");
include!("cp/memberref.rs");
include!("cp/methodref.rs");
include!("cp/symref.rs");
