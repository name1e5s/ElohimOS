#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate log;

mod boot;
mod config;
mod info;
mod page_table;

use alloc::boxed::Box;
use boot::*;
use config::*;
use info::*;
use uefi::prelude::*;
use uefi::table::boot::*;
use uefi::table::cfg::ACPI2_GUID;
use x86_64::registers::control::{Cr0, Cr0Flags, Cr3, Efer, EferFlags};
use x86_64::structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};
use xmas_elf::ElfFile;

/// The entry point of kernel, set by BSP.
static mut ENTRY: usize = 0;
/// Physical memory offset, set by BSP.
static mut PHYSICAL_MEMORY_OFFSET: u64 = 0;
//#[no_mangle]
//pub static _fltused: u32 = 0;

#[entry]
fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> Status {
    // Start initlize process
    uefi_services::init(&system_table).expect_success("failed to initialize utilities");
    info!("Hello UEFI!");

    // Get kernel path
    info!("Kernel Path: {}", DEFAULT_CONFIG.efi_path);
    let services = system_table.boot_services();

    // Init GOP mode
    let gop_info = init_graphics(services, DEFAULT_CONFIG.resolution);
    info!("Framebuffer Addr: {:x}", gop_info.framebuffer_addr);

    // Get ACPI 2 RSDP address
    let acpi_rsdp_addr = system_table
        .config_table()
        .iter()
        .find(|entry| entry.guid == ACPI2_GUID)
        .expect("ERR: ACPI 2 RSDP not found")
        .address;
    info!("ACPI RSDP: {:?}", acpi_rsdp_addr);

    // Open and read the KERNEL
    let elf = ElfFile::new(read_file(services, DEFAULT_CONFIG.efi_path))
        .expect("ERR: Invalid ELF format");

    // Fill the global variables
    unsafe {
        ENTRY = elf.header.pt2.entry_point() as usize;
        PHYSICAL_MEMORY_OFFSET = DEFAULT_CONFIG.physical_memory_address;
        info!("ELF ENTRY: {:x}", ENTRY);
    }

    // Init memory
    let max_mmap_size = system_table.boot_services().memory_map_size();
    let mmap_storage = Box::leak(vec![0; max_mmap_size].into_boxed_slice());
    let mmap_iter = system_table
        .boot_services()
        .memory_map(mmap_storage)
        .expect_success("failed to get memory map")
        .1;
    let max_phys_addr = mmap_iter
        .map(|m| m.phys_start + m.page_count * 0x1000)
        .max()
        .unwrap()
        .max(0x100000000);

    // Map KERNEL into page table
    let p4_table_addr = Cr3::read().0.start_address().as_u64();
    let p4_table = unsafe { &mut *(p4_table_addr as *mut PageTable) };
    let mut page_table = unsafe { OffsetPageTable::new(p4_table, VirtAddr::new(0)) };
    unsafe {
        Cr0::update(|f| f.remove(Cr0Flags::WRITE_PROTECT));
        Efer::update(|f| f.insert(EferFlags::NO_EXECUTE_ENABLE));
    }
    page_table::map_elf(&elf, &mut page_table, &mut UEFIFrameAllocator(services))
        .expect("ERR: Map ELF failed");

    // Map low addresses
    page_table::map_physical_memory(
        DEFAULT_CONFIG.physical_memory_address,
        max_phys_addr,
        &mut page_table,
        &mut UEFIFrameAllocator(services),
    );
    unsafe {
        Cr0::update(|f| f.insert(Cr0Flags::WRITE_PROTECT));
    }
    info!("exit boot services");
    // Exit boot services
    let (_rt, mmap_iter) = system_table
        .exit_boot_services(image, mmap_storage)
        .expect_success("Failed to exit boot services");

    // Construct device info for boot the KERNEL
    let device_info = DeviceInfo {
        memory_map: MemoryMap { iter: mmap_iter },
        physical_offset: DEFAULT_CONFIG.physical_memory_address,
        gop_info,
        acpi_rsdp_addr: acpi_rsdp_addr as u64,
    };

    // GO~
    jump_to_entry(&device_info);
}

fn jump_to_entry(device_info: *const DeviceInfo) -> ! {
    unsafe {
        asm!(  "add rsp, $0; jmp $1"
            :: "m"(PHYSICAL_MEMORY_OFFSET),
               "r"(ENTRY), 
               "{rdi}"(device_info)
            :: "intel");
    }
    unreachable!()
}

pub struct UEFIFrameAllocator<'a>(&'a BootServices);

unsafe impl FrameAllocator<Size4KiB> for UEFIFrameAllocator<'_> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let addr = self
            .0
            .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 1)
            .expect_success("ERR: Failed to allocate frame");
        Some(PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}
