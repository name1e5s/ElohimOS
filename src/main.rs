#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let frame_buffer = (0xFFFF800000000000u64 + 0x80000000u64) as *mut u64;

    for i in 0..768 {
        for j in 0..1024 {
            unsafe {
                *frame_buffer.offset(i as isize * 0x400 + j as isize) = 0xFF00FF00;
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
