//! 用户 shell 命令提示符
#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;

#[macro_use]
extern crate user_lib;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;

use alloc::string::String;
use user_lib::console::getchar;
use user_lib::{exec, fork, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    println!("Jumbuck Nucleus OS 绵羊核心操作系统 [Version 0.1.0]");
    // println!("\t基于 rust 语言 与 RISCV 架构的 非4KiB 页(PN4)");
    // println!("\t感谢 rCore 与 uCore");
    let mut line: String = String::new(); //动态字符串，添加字符。
    print!(">> ");
    loop {
        let c = getchar();
        match c {
            //换行符
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    line.push('\0'); //C语言字符串惯例。用于系统调用传递字符串。
                    let pid = fork();
                    if pid == 0 {
                        // child process
                        if exec(line.as_str()) == -1 {
                            println!("user_shell: \"{}\" 不是内部或外部指令，\n也不是可执行文件或者脚本，故无法执行！", line);
                            return -4;
                        }
                        unreachable!();
                    } else {
                        let mut exit_code: i32 = 0;
                        let exit_pid = waitpid(pid as usize, &mut exit_code);
                        assert_eq!(pid, exit_pid);
                        println!("user_shell: 进程 {} 以退出码 {} 结束。", pid, exit_code);
                    }
                    line.clear();
                }
                print!(">> "); //处理下一次。
            }
            //退格
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char); //特殊字符
                    print!(" ");//擦掉
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            //正常字符
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}
