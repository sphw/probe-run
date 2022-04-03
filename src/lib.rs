pub mod backtrace;
pub mod canary;
//mod cli;
pub mod cortexm;
pub mod dep;
pub mod elf;
//mod probe;
pub mod registers;
pub mod stacked;
pub mod target_info;

use std::time::Duration;

use crate::{elf::Elf, target_info::TargetInfo};

const TIMEOUT: Duration = Duration::from_secs(1);
