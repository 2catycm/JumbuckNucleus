//! RISC-V timer-related functionality

use riscv::register::time;
use crate::environment::board;
use crate::environment::sbi::sbi;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
///get current time
pub fn get_time() -> usize {
    time::read()
}
/// get current time in microseconds
pub fn get_time_ms() -> usize {
    time::read() / (board::CLOCK_FREQ / MSEC_PER_SEC)
}
/// set the next timer interrupt
pub fn set_next_trigger() {
    sbi::set_timer(get_time() + board::CLOCK_FREQ / TICKS_PER_SEC);
}
