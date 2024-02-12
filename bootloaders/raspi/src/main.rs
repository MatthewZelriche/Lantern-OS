#![no_std]
#![no_main]

use kernel::kmain;

pub fn kstart() -> ! {
    kmain()
}


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {

    loop {}
}