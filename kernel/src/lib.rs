#![no_std]

use device_drivers::character_device::CharacterDevice;

pub mod allocators;
pub mod device_drivers;
pub mod util;

pub fn kmain(writer: &'static mut dyn CharacterDevice) -> ! {
    writeln!(writer, "Hello from kernel land!");
    loop {}
}
