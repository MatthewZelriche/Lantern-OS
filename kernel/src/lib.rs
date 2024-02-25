#![no_std]
#![feature(coerce_unsized)]
#![feature(unsize)]

pub mod allocators;
pub mod device_drivers;
pub mod print;
pub mod util;

pub fn kmain() -> ! {
    kprint!("Hello from kernel");
    loop {}
}
