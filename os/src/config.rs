//! Constants used in rCore
pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;

// pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE: usize = 0x1000<<2; // 16Ki = 2^14 = 2^2 * (2^4)^3 = 0x4000
// pub const PAGE_SIZE_BITS: usize = 0xc;
pub const PAGE_SIZE_BITS: usize = 4+10; // 16KiB， 需要14个 bit

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

pub use crate::board::{CLOCK_FREQ, MEMORY_END, MMIO};
