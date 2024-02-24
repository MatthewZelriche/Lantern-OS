#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{arch::global_asm, ffi::c_void, fmt::Write, panic::PanicInfo};
use device_drivers::{
    gpio::{Gpio, GPIO_PHYS_BASE},
    uart0::{Pl011, PL011_PHYS_BASE},
};
use kernel::{
    allocators::static_bump::StaticBumpAlloc, kmain, read_linker_var,
    util::linker_variables::__PG_SIZE,
};

use crate::device_drivers::mailbox::{Mailbox, MAILBOX_PHYS_BASE};

mod device_drivers;
#[cfg(test)]
mod test;
mod util;

#[cfg(feature = "raspi3")]
static RASPI_VERSION: u8 = 3;
#[cfg(feature = "raspi4")]
static RASPI_VERSION: u8 = 4;

global_asm!(include_str!("main.S"));

#[no_mangle]
pub extern "C" fn bootloader_main(dtb_ptr: *const c_void) -> ! {
    // We don't have a memory map set up yet, but we know that the first page after the zero page will always
    // be free
    let static_mem_start = read_linker_var!(__PG_SIZE);
    let static_mem_size = read_linker_var!(__PG_SIZE);
    let mut static_alloc = unsafe { StaticBumpAlloc::new(static_mem_start, static_mem_size) };

    #[cfg(test)]
    test_main();

    // Create our drivers that we will use for the duration of the bootloader
    let mut gpio: Gpio;
    let mut mailbox: Mailbox;
    let mut uart: Pl011;
    unsafe {
        gpio = Gpio::new(GPIO_PHYS_BASE);
        mailbox = Mailbox::new(MAILBOX_PHYS_BASE);
        uart = Pl011::new(PL011_PHYS_BASE, &mut gpio);
    }

    let writer = static_alloc.allocate(uart).unwrap();
    writeln!(writer, "Transferring control from bootloader to kernel!");

    kmain(writer)
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
