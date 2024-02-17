#![no_std]
#![no_main]

use core::{arch::global_asm, ffi::c_void, panic::PanicInfo};
use device_drivers::{gpio::{Gpio, GPIO_PHYS_BASE}, uart0::Uart0};
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
    let mut uart: Uart0;
    unsafe {
        gpio = Gpio::new(GPIO_PHYS_BASE);
        uart = Uart0::new(&mut gpio);
    }

    kmain()
}


#[panic_handler]
fn panic(_: &PanicInfo) -> ! {

    loop {}
}