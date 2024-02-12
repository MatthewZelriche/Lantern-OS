#![no_std]
#![no_main]


fn kstart() -> ! {
    loop {}
}


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {

    loop {}
}