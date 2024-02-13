#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};
use kernel::kmain;

global_asm!(include_str!("main.S"));

#[no_mangle]
pub extern "C" fn bootloader_main() -> ! {
    kmain()
}


#[panic_handler]
fn panic(_: &PanicInfo) -> ! {

    loop {}
}