//! 通过系统调用ls列出所有用户进程。
#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::ls;
#[no_mangle]
pub fn main() -> i32 {
    ls();
    0
}