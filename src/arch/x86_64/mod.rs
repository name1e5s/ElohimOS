#[macro_use]
mod driver;
mod gdt;

use core::panic::PanicInfo;
use bootloader::info::*;

use uefi::table::boot::MemoryType;

#[no_mangle]
pub extern "C" fn _start(info: &'static DeviceInfo) -> ! {
    driver::serial_init();
    info!("Hello world from elohim");
    info!("Init gdt...");
    gdt::init();
    print_physical_memory(info);
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

fn print_physical_memory(info: &DeviceInfo) {
    for region in info.memory_map.clone().iter {
        if region.ty == MemoryType::CONVENTIONAL {
            let start_addr = region.phys_start as usize;
            let end_addr = start_addr + region.page_count as usize * 0x1000;
            info!("Physical memory range: [{:x}, {:x}]", start_addr, end_addr);
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}