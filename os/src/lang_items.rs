//! The panic handler
use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let (Some(location), Some(message))= (info.location(), info.message()) {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            message
        );
    } else {
        println!("[kernel] Panicked! ");
    }
    shutdown()
}
