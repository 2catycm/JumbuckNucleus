//! # 绵羊核心操作系统（OS）
//! 1. 内核的代码从`entry.asm`开始，然后 [`rust_main()`] 被调用、
//!
//! 2. 重要的OS子功能在子模块当中实现。 我们通过初始化子模块，
//! - 启动了操作系统的基本功能。
//! - 对操作系统的功能进行自检。
//!
//! 这些子模块是：
//!
//! - [`trap`]: 处理从用户态转移到内核态的三种异常情况（异常、系统调用、中断）
//!
//! - [`task`]:  任务（进程）管理。
//! - [`syscall`]: 系统调用的接管
//!
//! - [`mm`]:  基于 SV39 的内存管理方案
//! - [`sync`]:  UPSafeCell 声明在单进程下访问为安全，避免 unsafe 块的使用。
//! 3. 当操作系统初始化完毕后，我们使用[`task::run_tasks()`]运行用户进程，进入受限直接执行的进程管理。
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

#[cfg(feature = "board_k210")]
#[path = "boards/k210.rs"]
mod board;
#[cfg(not(any(feature = "board_k210")))]
#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
pub mod mm;
mod sbi;
pub mod sync;
pub mod syscall;
pub mod task;
mod timer;
pub mod trap;
// mod sheep_logger;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));
/// clear BSS segment
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
/// the rust entry-point of os
pub fn rust_main() -> ! {
    clear_bss();
    println!("欢迎来到，绵羊核心。");
    mm::init();

    mm::remap_test();
    task::add_initproc();
    println!("after initproc!");
    trap::init();
    //trap::enable_interrupt();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    loader::list_apps();
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}
