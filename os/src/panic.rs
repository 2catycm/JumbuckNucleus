//! panic 库，实现 panic 和 abort 的基本功能。

use crate::sbi;
use crate::console;
/// 打印 panic 信息并 [`sbi::shutdown`].
///
/// ### '#[panic_handler]' 属性
/// 声明此函数是 panic 的回调。
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    console::open_blue_print();
    let repeated_space = "                                                                                                                    ";
    println!(":(\n你的电脑遇到问题，需要关机。");
    println!("我们只为您收集关键信息，然后为您关机。");
    let mut i = 10;
    for _ in 0..4 {
        println!("{}%{}", i, repeated_space);
        i+=10;
    }
    println!("\t如果您需要了解更多信息，请查看此错误:");
    if let Some(location) = info.location() {
        println!(
            "\t{}:{} \n\t\t'{:?}'",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("{}", info.message().unwrap());
    }
    for _ in 0..6 {
        println!("{}%{}", i, repeated_space);
        i+=10;
    }
    console::close_console_effects();
    sbi::shutdown();
}

/// abort 函数
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()")
}
