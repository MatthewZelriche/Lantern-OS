#![no_std]
#![no_main]

use core::{arch::global_asm, ffi::c_void, fmt::Write, panic::PanicInfo};
use device_drivers::{
    gpio::{Gpio, GPIO_PHYS_BASE},
    uart0::{Pl011, PL011_PHYS_BASE},
};
use kernel::kmain;

mod device_drivers;

#[cfg(feature = "raspi3")]
static RASPI_VERSION: u8 = 3;
#[cfg(feature = "raspi4")]
static RASPI_VERSION: u8 = 4;

global_asm!(include_str!("main.S"));

#[no_mangle]
pub extern "C" fn bootloader_main(dtb_ptr: *const c_void) -> ! {
    // Create our drivers that we will use for the duration of the bootloader
    let mut gpio: Gpio;
    let mut uart: Pl011;
    unsafe {
        gpio = Gpio::new(GPIO_PHYS_BASE);
        uart = Pl011::new(PL011_PHYS_BASE, &mut gpio);
    }

    writeln!(uart, "Transferring control from bootloader to kernel...");
    kmain()
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
