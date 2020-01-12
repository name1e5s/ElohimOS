#![no_std]
#![feature(asm)]

pub use uefi::proto::console::gop::ModeInfo;
pub use uefi::table::boot::{MemoryMapIter, MemoryDescriptor, MemoryType, MemoryAttribute};

pub mod info;