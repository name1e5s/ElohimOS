use core::fmt;
use uefi::proto::console::gop::ModeInfo;
use uefi::table::boot::MemoryMapIter;

/// Pass this device info to the kernel we boot.
#[repr(C)]
#[derive(Debug)]
pub struct DeviceInfo {
    pub memory_map: MemoryMap,
    pub physical_offset: u64,
    pub gop_info: GOPInfo,
    pub acpi_rsdp_addr: u64,
}

/// Graphics info of the device
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GOPInfo {
    pub mode: ModeInfo,
    pub framebuffer_addr: u64,
    pub framebuffer_size: u64,
}

pub struct MemoryMap {
    pub iter: MemoryMapIter<'static>,
}

impl Clone for MemoryMap {
    fn clone(&self) -> Self {
        unsafe { core::ptr::read(self) }
    }
}

impl fmt::Debug for MemoryMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_list();
        for mmap in self.clone().iter {
            f.entry(mmap);
        }
        f.finish()
    }
}
