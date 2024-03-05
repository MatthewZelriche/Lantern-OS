#![no_main]
#![no_std]

use core::panic::PanicInfo;

pub mod print;

// no_mangle is necessary to stop this fn from being optimized out
#[link_section = ".text.boot"]
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("\n");
    kprintln!("KERNEL PANIC!");
    kprintln!("{}", info);
    loop {}
}
