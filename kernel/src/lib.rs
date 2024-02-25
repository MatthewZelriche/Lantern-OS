#![no_std]
#![feature(coerce_unsized)]
#![feature(unsize)]

use core::panic::PanicInfo;

pub mod allocators;
pub mod device_drivers;
pub mod print;
pub mod util;

pub fn kmain() -> ! {
    kprintln!("Hello from kernel");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("\n");
    kprintln!("KERNEL PANIC!");
    kprintln!("{}", info);
    loop {}
}
