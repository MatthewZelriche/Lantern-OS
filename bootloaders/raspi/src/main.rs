#![no_std]
#![no_main]

use core::{arch::{asm, global_asm}, ffi::c_void, panic::PanicInfo};
use kernel::kmain;

#[cfg(feature = "raspi3")]
static RASPI_VERSION: u8 = 3;
#[cfg(feature = "raspi4")]
static RASPI_VERSION: u8 = 4;

global_asm!(include_str!("main.S"));

#[no_mangle]
pub extern "C" fn bootloader_main(dtb_ptr: *const c_void) -> ! {  
    unsafe { asm!("mov x21, {rpi}", rpi = in(reg) RASPI_VERSION as u64); }
    kmain()
}


#[panic_handler]
fn panic(_: &PanicInfo) -> ! {

    loop {}
}