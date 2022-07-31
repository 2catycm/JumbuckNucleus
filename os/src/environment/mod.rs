#[cfg(feature = "board_k210")]
#[path = "boards/k210.rs"]
pub(crate) mod board;
#[cfg(not(any(feature = "board_k210")))]
#[path = "boards/qemu.rs"]
mod board;

pub(crate) mod sbi;
#[macro_use]
pub(crate) mod lang_items;
