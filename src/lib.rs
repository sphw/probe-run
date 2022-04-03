mod backtrace;
mod canary;
//mod cli;
mod cortexm;
mod dep;
mod elf;
//mod probe;
mod registers;
mod stacked;
mod target_info;

use std::time::Duration;

use crate::{backtrace::Outcome, canary::Canary, elf::Elf, target_info::TargetInfo};

const TIMEOUT: Duration = Duration::from_secs(1);
