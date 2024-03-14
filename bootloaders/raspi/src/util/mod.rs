use crate::println;
use core::panic::PanicInfo;

pub mod print;

#[cfg(feature = "raspi3")]
static RASPI_VERSION: u8 = 3;
#[cfg(feature = "raspi4")]
static RASPI_VERSION: u8 = 4;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("");
    println!("BOOTLOADER PANIC! Reason: ");
    println!("{}", info);
    loop {}
}
