#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{arch::global_asm, ffi::c_void};
use device_drivers::{
    gpio::{Gpio, GPIO_PHYS_BASE},
    uart0::{Pl011, PL011_PHYS_BASE},
};
use kernel::{
    allocators::{static_box::StaticBox, static_bump::StaticBumpAlloc},
    kmain, kprintln,
    print::{self, GlobalWriter, GLOBAL_WRITER},
    read_linker_var,
    util::linker_variables::__PG_SIZE,
};
use writer_mutexes::single_threaded::SingleThreadedRawWriterMutex;

use crate::device_drivers::mailbox::{
    message::{SetClockRate, CLOCK_UART},
    Mailbox, MAILBOX_PHYS_BASE,
};

mod device_drivers;
#[cfg(test)]
mod test;
mod writer_mutexes;

#[cfg(feature = "raspi3")]
static RASPI_VERSION: u8 = 3;
#[cfg(feature = "raspi4")]
static RASPI_VERSION: u8 = 4;

global_asm!(include_str!("main.S"));

#[no_mangle]
pub extern "C" fn bootloader_main(_dtb_ptr: *const c_void) -> ! {
    // Performs critical early initialization that must be done before we can do virtually anything else,
    // including before we can use print macros at all. Returns a static bump allocator that was created
    // for the purposes of setting up the global writer.
    let _static_alloc = early_init();

    #[cfg(test)]
    test_main();

    kprintln!("Transferring control from bootloader to kernel...");
    kmain()
}

fn early_init() -> StaticBumpAlloc {
    let static_mem_start = read_linker_var!(__PG_SIZE);
    let static_mem_size = read_linker_var!(__PG_SIZE);
    let mut static_alloc = unsafe {
        // Safety: We know that the first page after the zero page will always be free, so we can assign that
        // block of memory to this bump allocator, so long as we never use it for anything else.
        StaticBumpAlloc::new(static_mem_start, static_mem_size)
    };

    // Create our drivers that we will use for the duration of the bootloader
    let mut gpio: Gpio;
    let mut mailbox: Mailbox;
    let uart: Pl011;
    unsafe {
        // Safety: The MMIO addresses are correct for the given Raspberry Pi board revision.
        gpio = Gpio::new(GPIO_PHYS_BASE);
        mailbox = Mailbox::new(MAILBOX_PHYS_BASE);
    }

    // Set the UART frequency to a known value
    let mut uart_rate_msg = SetClockRate::new(CLOCK_UART, 30000000);
    mailbox.send_property_mail(&mut uart_rate_msg).unwrap();
    unsafe {
        // Safety: The MMIO address is correct and we have set the correct UART clock frequency
        uart = Pl011::new(PL011_PHYS_BASE, &mut gpio);
    }

    // Construct the Single Threaded Global Writer we will use until we enable the MMU.
    let static_uart = StaticBox::new(uart, &mut static_alloc).unwrap();
    unsafe {
        // Safety: We are in a single-threaded environment
        let st_writer_mutex =
            StaticBox::new(SingleThreadedRawWriterMutex::new(), &mut static_alloc).unwrap();
        GLOBAL_WRITER.set(GlobalWriter::new(static_uart, st_writer_mutex));
    }

    static_alloc
}
