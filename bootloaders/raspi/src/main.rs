#![no_std]
#![no_main]

use core::{arch::global_asm, ffi::c_void, panic::PanicInfo};
use kernel::kmain;

global_asm!(include_str!("main.S"));

#[no_mangle]
pub extern "C" fn bootloader_main(dtb_ptr: *const c_void, rpi_version: u8) -> ! {  
    kmain()
}


#[panic_handler]
fn panic(_: &PanicInfo) -> ! {

    loop {}
}