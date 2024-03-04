#![no_main]
#![no_std]

use core::panic::PanicInfo;

pub mod print;

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
