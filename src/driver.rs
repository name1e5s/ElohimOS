pub mod serial;

use core::fmt::{self, Write};
use log::*;
use serial::Serial;

static mut COM1: Serial = unsafe { Serial::new(0x3F8) };

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::driver::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    unsafe {
        COM1.write_fmt(args).unwrap();
    }
}

pub struct Logger {}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        println!("{}: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

static mut LOGGER: Logger = Logger {};
pub fn serial_init() {
    unsafe {
        COM1.init();
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
    }
}
