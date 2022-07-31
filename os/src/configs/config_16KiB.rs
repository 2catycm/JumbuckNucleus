//! Constants used in rCore

pub const PAGE_SIZE_BITS: usize = include!("16KiB.conf"); // 16KiB， 需要14个 bit

pub const PAGE_SIZE: usize = 1<<PAGE_SIZE_BITS;
pub const HUGE_PAGE_SIZE: usize = PAGE_SIZE *512;

pub const USER_STACK_SIZE: usize = PAGE_SIZE*8;
pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE*8;
pub const KERNEL_HEAP_SIZE: usize = HUGE_PAGE_SIZE;


pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

pub use crate::board::{CLOCK_FREQ, MEMORY_END, MMIO};
