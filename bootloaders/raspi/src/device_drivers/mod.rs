pub mod gpio;
pub mod mailbox;
pub mod uart0;

#[cfg(feature = "raspi3")]
pub const MMIO_BASE: usize = 0x3F000000;
#[cfg(feature = "raspi4")]
// "a peripheral described in this document ... [is] visible to the ARM at 0x0_FEnn_nnnn if Low Peripheral
// mode is enabled."
// - BCM2711 ARM PERIPHERALS
pub const MMIO_BASE: usize = 0xFE000000;
#[cfg(not(any(feature = "raspi3", feature = "raspi4")))]
compile_error!("One of raspi3 or raspi4 must be enabled!.");
