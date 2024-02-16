#[cfg(feature = "raspi3")]
pub static MMIO_BASE: u64 = 0x3F000000;
#[cfg(feature = "raspi4")]
pub static MMIO_BASE: u64 = 0xFC000000;
#[cfg(not(any(feature = "raspi3", feature = "raspi4")))]
compile_error!("One of raspi3 or raspi4 must be enabled!.");