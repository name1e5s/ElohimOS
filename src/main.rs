#![no_std]
#![no_main]

#[macro_use]
extern crate x86_64;

#[macro_use]
extern crate log;

extern crate bootloader;

#[macro_use]
mod driver;

use core::panic::PanicInfo;
use bootloader::info::DeviceInfo;

#[no_mangle]
pub extern "C" fn _start(info: &'static DeviceInfo) -> ! {
    driver::serial_init();
    info!("Hello world from elohim");

    let frame_buffer = (0xFFFF800000000000u64 + 0x80000000u64) as *mut u32;

    for i in 0..768 {
        for j in 0..1024 {
            let color: u32 = match (i >> 5) % 4 {
                0 => 0xFF000000,
                1 => 0xFFFF0000,
                2 => 0xFF00FF00,
                3 => 0xFF0000FF,
                _ => unreachable!(),
            };
            unsafe {
                *frame_buffer.offset(i as isize * 0x400 + j as isize) = color;
            }
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
