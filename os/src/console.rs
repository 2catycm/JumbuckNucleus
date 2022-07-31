//! 实现控制台的字符输入和输出
//!
//! # 格式化输出
//!
//! [`core::fmt::Write`] trait 包含
//! - 需要实现的 [`write_str`] 方法
//! - 自带实现，但依赖于 [`write_str`] 的 [`write_fmt`] 方法。
//!
//! 我们声明一个类型，并为其实现 [`write_str`] 方法后，就可以使用 [`write_fmt`] 来进行格式化输出。
//!
//! [`write_str`]: core::fmt::Write::write_str
//! [`write_fmt`]: core::fmt::Write::write_fmt
#![allow(unused)]
use crate::sbi::*;
use core::fmt::{self, Write};

/// 一个 [Zero-sized Type], 实现 [`core::fmt::Write`] trait 来进行格式化输出。
struct Stdout;

impl Write for Stdout {
    /// 标准输出流的打印字符串操作实现。
    ///
    /// ['console_putchar'] sbi 调用每次接受一个 `usize`, 但实际上其并不是一个 unicode 式的输出接口
    /// 想要正确地输出一个非 ASCII 字符，需要将其转义成一个 utf-8 序列并逐个进行输出。
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            console_putchar(b.into());
        }
        Ok(())
    }
}

/// 打印由 [`core::format_args!`] 格式化后的数据。
///
/// `print!` 和 `println!` 宏都将展开成此函数。
///
/// 未知的引用：
/// [`core::format_args!`]: <https://doc.rust-lang.org/nightly/core/macro.format_args.html>
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// 实现类 std 的宏 print!
///
/// 使用实现了 [`core::fmt::Write`] trait 的 `console::Stdout`.
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($args)+)?));
    }
}

/// 实现类 std 的宏 println!
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")  //如果是空的，没有参数，那么调用上面的print打出一行。
    };
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}

/// 实现类 std 的宏 eprint!
#[macro_export]
macro_rules! eprint {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!(concat!("{}", $fmt), "{}"),
        "\x1b[1;31m" $(, $($arg)+)? , "\x1b[0m"))
    }
}

/// 实现类 std 的宏 eprintln!
#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::eprint!("\n")  //如果是空的，没有参数，那么调用上面的eprint打出一行。
    };
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!(concat!("{}", concat!($fmt, "\n")), "{}"),
        "\x1b[1;31m" $(, $($arg)+)? , "\x1b[0m"))
    }
}

/// 实现 简单彩色打印
#[macro_export]
macro_rules! color_print {
    ($color:expr, $fmt: literal $(, $($arg: tt)+)?) => {
        $print(format_args!(concat!(concat!("\x1b[{}m", $fmt), "\x1b[0m"),
        $color $(, $($arg)+)? ))
    }
}
/// 实现简单彩色打印换行输出
#[macro_export]
macro_rules! color_println {
    ($color:expr) => {
        $crate::color_print!($color, "\n")  //如果是空的，没有参数，那么调用上面的eprint打出一行。
    };
    ($color:expr, $fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!(concat!("\x1b[{}m", $fmt), "\x1b[0m\n"),
        $color $(, $($arg)+)? ))
    }
}

/// 开启蓝屏
pub fn open_blue_print() {
    print!("\x1b[44m");
}
/// 清除控制台效果
pub fn close_console_effects() {
    print!("\x1b[0m");
}
/// 清空控制台
pub fn clear_console() {
    print!("\x1b[2J");
}
