//! 用户程序 bench_pgfault
//! 测试16KiB分页方案是否有效
#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::vec::Vec;
use core::mem;
use core::f32;
use core::f64;
use user_lib::getpid;
use user_lib::get_time;


#[no_mangle]
pub fn main() -> i32 {
    println!("分页性能测试");
    let start = get_time();
    let mut v: Vec<i32> = Vec::new();
    let l = 16 * 1024 / mem::size_of::<i32>() / 2;
    // 在一个16KiB中反复读写。
    for i in 0..100 {
        for j in 0..l {
            // v.push(((((j as f32)/2.5 + 1.2)*(-1.25)+3.0) / 3.4) as i32) ;
            v.push((10 * i + j - (i * j)) as i32) ;
        }
        v.sort();
    }
    println!("测试结束，用时{}ms", get_time() - start);
    0
}
