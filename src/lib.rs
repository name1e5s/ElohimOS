#![no_std]

#[allow(dead_code)]
#[path = "arch/x86_64/mod.rs"]
pub mod arch;

extern crate x86_64;
extern crate uefi;

#[macro_use]
extern crate log;

extern crate bootloader;