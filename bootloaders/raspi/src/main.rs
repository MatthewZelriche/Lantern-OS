#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{arch::global_asm, ffi::c_void, panic::PanicInfo};
use device_drivers::{
    gpio::{Gpio, GPIO_PHYS_BASE},
    uart0::{Pl011, PL011_PHYS_BASE},
};
use kernel::{
    allocators::{static_box::StaticBox, static_bump::StaticBumpAlloc},
    kmain, print,
    print::{GlobalWriter, GLOBAL_WRITER},
    read_linker_var,
    util::linker_variables::__PG_SIZE,
};
use mutexed_writers::single_threaded_mutexed_writer::SingleThreadedMutexedWriter;

use crate::device_drivers::mailbox::{Mailbox, MAILBOX_PHYS_BASE};

mod device_drivers;
mod mutexed_writers;
#[cfg(test)]
mod test;

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

    // Create our drivers that we will use for the duration of the bootloader
    let mut gpio: Gpio;
    let mut mailbox: Mailbox;
    let mut uart: Pl011;
    unsafe {
        gpio = Gpio::new(GPIO_PHYS_BASE);
        mailbox = Mailbox::new(MAILBOX_PHYS_BASE);
        uart = Pl011::new(PL011_PHYS_BASE, &mut gpio);
    }

    // Construct the Single Threaded Global Writer we will use until we enable the MMU.
    let st_writer =
        StaticBox::new(SingleThreadedMutexedWriter::new(uart), &mut static_alloc).unwrap();
    unsafe {
        // Safety: We are in a single-threaded environment
        GLOBAL_WRITER.set(GlobalWriter::new(st_writer));
    }

    print!("Hello from Raspi {} bootloader\n", RASPI_VERSION);

    #[cfg(test)]
    test_main();

    kmain()
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
