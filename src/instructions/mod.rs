pub mod comparisons;
pub mod constants;
pub mod control;
pub mod conversions;
pub mod extended;
pub mod loads;
pub mod math;
pub mod references;
pub mod reserved;
pub mod stack;
pub mod stores;

use comparisons::*;
use constants::*;
use control::*;
use conversions::*;
use extended::*;
use loads::*;
use math::*;
use stack::*;
use stores::*;
use references::*;

use crate::rtda::Frame;
use std::fmt::Debug;
use crate::rtda::heap::Method;

include!("instruction.rs");
include!("bytecode_reader.rs");
include!("branch_instruction.rs");
include!("method_invoke_logic.rs");
