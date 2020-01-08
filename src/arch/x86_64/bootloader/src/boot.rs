use crate::info::*;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::file::*;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::*;

pub fn init_graphics(service: &BootServices, resolution: (usize, usize)) -> GOPInfo {
    let output = service
        .locate_protocol::<GraphicsOutput>()
        .expect_success("ERR: GraphicsOutput is not accessiable.");
    let output = unsafe { &mut *output.get() };

    let mode = output
        .modes()
        .map(|mode| {
            let mode = mode.expect("Query failed");
            mode
        })
        .find(|ref mode| mode.info().resolution() == resolution)
        .expect("ERR: Graphic mode not found");
    output
        .set_mode(&mode)
        .expect_success("ERR: Failed to set VGA mode");
    info!("Set VGA resolution to {:?}", resolution);
    GOPInfo {
        mode: output.current_mode_info(),
        framebuffer_addr: output.frame_buffer().as_mut_ptr() as u64,
        framebuffer_size: output.frame_buffer().size() as u64,
    }
}

/// Read a file into memory.
pub fn read_file(service: &BootServices, path: &str) -> &'static mut [u8] {
    info!("Opening file {}", path);
    let fs = service
        .locate_protocol::<SimpleFileSystem>()
        .expect_success("ERR: FileSystem not found");
    let fs = unsafe { &mut *fs.get() };

    let mut volume = fs.open_volume().expect_success("ERR: Open volume failed");
    let handle = volume
        .open(path, FileMode::Read, FileAttribute::empty())
        .expect_success("ERR: Open file failed.");
    let mut file = match handle
        .into_type()
        .expect_success("ERR: Failed to into_type")
    {
        FileType::Regular(regular) => regular,
        _ => panic!("ERR: Invalid file type"),
    };

    info!("Loading file into memory");
    let mut file_info = [0u8; 0x100];
    let file_info = file
        .get_info::<FileInfo>(&mut file_info)
        .expect_success("ERR: Failed to get file info");
    let page_count = file_info.file_size() as usize / 0x1000 + 1;
    let mem_base = service
        .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, page_count)
        .expect_success("ERR: Failed to allocate pages");
    let buf = unsafe { core::slice::from_raw_parts_mut(mem_base as *mut u8, page_count * 0x1000) };
    let len = file.read(buf).expect_success("ERR: Read file failed");
    &mut buf[..len]
}
