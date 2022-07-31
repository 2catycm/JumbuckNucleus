use crate::color_println;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

/// Implements [`Log`].
pub struct SheepLogger {}
#[derive(Debug)]
enum LogColor {
    Red = 31,
    Green = 32,
    Blue = 34,
    Gray = 90,
    Yellow = 93,
}

impl Log for SheepLogger {
    /// 为log_enabled!宏决定是否要对执行log，防止实际上不需要log的计算发生。
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true //我们已经设置了全局 level 。暂时不需要更加细化的 level 。
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if !self.enabled(record.metadata()) {
                return;
            }
            let color = match record.level() {
                Level::Error => LogColor::Red,
                Level::Warn => LogColor::Yellow,
                Level::Info => LogColor::Blue,
                Level::Debug => LogColor::Green,
                Level::Trace => LogColor::Gray,
            };
            color_println!(
                color as u32,
                "[{}] \n{}:\t{}",
                record.target(),
                record.level(),
                record.args()
            );
            //还有别的信息可以打
            //见 https://docs.rs/log/0.4.17/log/struct.Record.html
        }
    }
    fn flush(&self) {}
}
static LOGGER: SheepLogger = SheepLogger {};

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
pub fn set_level(level: LevelFilter) -> () {
    log::set_max_level(level);
    if cfg!(feature = "test_log") {
        test_can_run();
    }
}

// #[cfg(test_log)]
pub fn test_can_run() -> () {
    log::error!("This is an error message.");
    log::warn!("This is an warning message.");
    log::info!("This is an info message.");
    log::debug!("This is an warning message.");
    log::trace!("This is an trace message.");
    // ..
    log::info!("打印字母信息测试!");
    for c in 'A'..='Z' {
        eprint!("{}", c);
    }
    eprintln!();
}
