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

use crate::rtda::Frame;

include!("instruction.rs");
include!("bytecode_reader.rs");
include!("no_operands_instruction.rs");
include!("branch_instruction.rs");
include!("index8_instruction.rs");
include!("index16_instruction.rs");
