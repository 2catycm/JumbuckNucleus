//! 用户程序 Hello World
//! 测试的系统调用：println(write char), getpid
#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::getpid;

#[no_mangle]

pub fn main() -> i32 {
    println!("pid {}: Hello world from user mode program! 来自用户程序的问候！", getpid());
    0
}
