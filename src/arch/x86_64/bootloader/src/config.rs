#[derive(Debug)]
pub struct Config<'a> {
    pub physical_memory_address: u64,
    pub efi_path: &'a str,
    pub resolution: (usize, usize),
}

pub const DEFAULT_CONFIG: Config = Config {
    physical_memory_address: 0xFFFF800000000000,
    efi_path: "\\EFI\\Elohim\\Elohim.efi",
    resolution: (1024, 768),
};
